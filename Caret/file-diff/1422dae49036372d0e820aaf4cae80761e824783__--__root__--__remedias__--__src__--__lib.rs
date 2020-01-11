//! # Remedias
//!
//! Remedias je centrální server sítě Meedias.
//! Spravuje landing page, endpointy API, hosting
//! statických souborů a administraci.
//!
//! V současnosti Remedias využívá sílu frameworku
//! Rocket pro web a Diesel jako ORM.
//!
//! Protože je Remedias poměrně velký projekt, je
//! pro jeho údržbu přiložen malý nástroj, `sentinel`.
//! Pro více informací si přečtěte jeho vlastní
//! dokumentaci.

#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel_migrations;
extern crate rocket_contrib;
extern crate rocket_cors;
#[macro_use] extern crate rocket;

extern crate diesel;
extern crate walkdir;

// our crates
#[macro_use] pub extern crate reapi;
extern crate statistics as _statistics;
extern crate category as _category;
extern crate instance as _instance;
extern crate content as _content;
extern crate query as _query;
extern crate tags as _tags;
extern crate page as _page;
extern crate user as _user;

#[macro_use] extern crate relog;
#[macro_use] pub mod log {
	//! Logování. Tento modul obsahuje makra
	//! ke sprostředkování asynchronního (ale bezpečného)
	//! logování. Kromě makra tu je také fairing pro Rocket,
	//! který automaticky loguje příchozí požadavky
	//!
	//! VAROVÁNÍ: tato knihovna musí být importována způsobem
	//! `extern crate relog as log`, jinak nebude log! makro fungovat
	pub use relog::*;
}

extern crate reutils;
pub mod util {
	//! Tento modul obsahuje utilitní funkce.
	//! Původně byly rozházeny do `api` modulů,
	//! ale v rámci organizace všechny přesunuty
	//! sem.
	pub use reutils::*;
}

extern crate remail;
pub mod email {
	//! Modul obsahující vše, co se týká emailů.
	//!
	//! A toho moc není...
	pub use remail::*;
}

extern crate redb;
pub mod db {
	//! Tento modul obsahuje vše, co se týká databáze,
	//! tj. modely, schéma a funkce pro komunikaci s Postgresem.
	//! V budoucnosti je v plánu tento modul generalizovat
	//! a přidat podporu pro SQLite (kvůli jednoduššímu testování)
	pub use redb::*;
}

pub extern crate restatic;

// TODO unify status for authentifications
// TODO fix reponder docs
// TODO responder macros for loc reduction
pub mod api;

#[cfg(test)]
mod tests;

use rocket::http::*;
use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

use std::str::FromStr;
use std::path::PathBuf;

/// experimentální cors funkce
#[options("/<_n..>")]
pub fn cors<'a>(_n: PathBuf) -> rocket::response::Result<'a> {
	rocket::Response::build()
		.status(Status::NoContent)
		.raw_header("Content-Type", "text/html; charset=utf-8")
		.raw_header("Server", "Rocket")
		.raw_header("Access-Control-Allow-Methods", "OPTIONS, PATCH, DELETE, TRACE, PUT, GET, POST")
		.raw_header("Access-Control-Allow-Origin", "http://localhost:9000")
		.raw_header("Access-Control-Allow-Headers", "content-type")
		.raw_header("Access-Control-Allow-Credentials", "true")
		.raw_header("Vary", "*")
		.raw_header("Content-Length", "0")
		.ok()
}

/// Vytvoří novou instanci
#[cfg_attr(feature = "cargo-clippy", allow(match_bool))]
pub fn init() -> rocket::Rocket {
	log!(info, high, "launching remedias");

	let cors = rocket_cors::Cors {
		allowed_origins: AllowedOrigins::all(),
		allowed_methods: vec![
			"options",
			"delete",
			"patch",
			"trace",
			"post",
			"get",
			"put",
		].into_iter().map(|s| FromStr::from_str(s).unwrap_or_else(|_| unreachable!("HTTP methods must be correct"))).collect(),
		allowed_headers: AllowedHeaders::all(),
		allow_credentials: true,
		..Default::default()
	};

	rocket::ignite().mount("/", routes![
		restatic::static_server::docs,
		restatic::static_server::handle,
		restatic::static_server::docs_redirect,
		restatic::landing::hmm,
		restatic::landing::send,
		restatic::landing::about,
		restatic::landing::index,
		restatic::landing::contacts,
		restatic::landing::services,
		api::category::children,
		api::category::get,
		api::category::post,
		api::category::delete,
		api::category::patch_slug,
		api::category::patch_name,
		api::category::patch_parent,
		api::category::post_perms,
		api::category::patch_perms,
		api::category::delete_perms,
		api::category::categories_get,
		api::category::categories_post,
		api::category::categories_patch,
		api::category::categories_delete,
		api::content::get,
		api::content::post,
		api::content::delete,
		api::content::publish,
		api::content::get_name,
		api::content::unpublish,
		api::content::patch_name,
		api::content::get_location,
		api::content::patch_location,
		api::content::delete_location,
		api::content::get_description,
		api::content::patch_description,
		api::content::get_image_url,
		api::content::patch_image_url,
		api::content::get_date,
		api::content::patch_date,
		api::content::get_text,
		api::content::patch_text,
		api::content::get_history,
		api::content::get_map,
		api::content::post_map,
		api::content::delete_map,
		api::page::get,
		api::page::post,
		api::page::patch,
		api::page::delete,
		api::page::get_ids,
		api::page::get_grid,
		api::page::post_grid,
		api::page::patch_grid,
		api::page::delete_grid,
		api::page::get_layouts,
		api::page::get_grid_by_url,
		api::user::get_me,
		api::user::get_nick,
		api::user::get_email,
		api::user::get_display,
		api::user::get_location,
		api::user::get_birthdate,
		api::user::login,
		api::user::check,
		api::user::refresh,
		api::user::invalidate,
		api::user::delete,
		api::user::confirm,
		api::user::register,
		api::user::i_delete,
		api::user::patch_nick,
		api::user::patch_email,
		api::user::patch_display,
		api::user::get_perms,
		api::user::post_perms,
		api::user::patch_perms,
		api::user::delete_perms,
		api::user::patch_location,
		api::user::delete_location,
		api::user::patch_birthdate,
		api::instance::get,
		api::instance::post,
		api::instance::delete,
		api::instance::get_owners,
		api::instance::patch_owners,
		api::instance::delete_owners,
		api::instance::get_name,
		api::instance::patch_name,
		api::instance::get_domain,
		api::instance::patch_domain,
		api::query::content_all,
		api::query::content_tag,
		api::query::content_date,
		api::query::content_random,
		api::query::content_category,
		api::query::content_unpublished,
		api::query::content_date_offset,
		api::query::content_category_children,
		api::query::category_by_slug,
		api::query::category_by_name,
		api::query::category_by_owner,
		api::query::category_by_parent,
		api::query::instance_random,
		api::query::instance_by_tag,
		api::query::instance_by_name,
		api::query::instance_by_owner,
		api::query::instance_by_domain,
		api::statistics::users,
		api::statistics::content,
		api::statistics::confirmed,
		api::statistics::instances,
		api::statistics::categories,
		api::statistics::page_fragments,
		api::query::user_by_nick,
		api::query::user_by_email,
		api::query::user_by_display,
		api::query::user_by_instance,
		api::tags::g_get,
		api::tags::g_post,
		api::tags::g_patch,
		api::tags::g_delete,
		api::tags::get,
		api::tags::post,
		api::tags::patch,
		api::tags::delete])
	.attach(AdHoc::on_launch("Environment Switcher", |rocket| match rocket.config().environment.is_dev() {
		true  => util::append_path("target/debug").unwrap_or_else(|_| unreachable!()),
		false => util::append_path("target/release").unwrap_or_else(|_| unreachable!()),
	}))
	.attach(Template::fairing())
	.attach(cors)
	.attach(db::fairing())
	.attach(log::Logger)
}

