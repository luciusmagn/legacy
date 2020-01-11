# Markdown Support

This document contains examples of most of Markdown elements that Caret supports.

[[toc]]

## Inlines

This is an _emphasis_ and this a strong **emphasis**. This is a [link](http://example.com) and this is an ![image](example.png). This is a [reference link](sth) and this is a URL: http://example.com. This is a `code span` and this is a math expression: $e=mc^2$. This is ::highlight:: and this is ~~strikethrough~~. This is a [reference link][sth] and this is some <span style="color: #d59">inline html</span>.

[sth]: http:example.com

Note that \*this* is not an emphasis. That's because the `\` character escapes special characters.

Most inline elements can be nested. For example this is a [link that contains a _emphasis_](http://example.com) and this is a ~~strikethrough that contains a [reference link][sth]~~.

## Leaf Blocks 🍁

A line consisting of 0-3 spaces of indentation, followed by a sequence of three or more matching `-`, `_`, or `*`, each followed optionally by any number of spaces, forms a *thematic break*.

* * *

This is an indented code block:

    var text = "text";
    function doNothing() {
      return;
    }

Fenced code blocks support syntax highlighting for most of the popular programming languages:

``` js
var text = "text"; // variable definition
```

HTML block:

<div style='background: #eea; padding: 10px 15px;'>
  This will appear as a block of with a yellow background.
</div>

This is a HTML comment:

<!-- this can be a note or a comment or sth else that i don't want to appear in the final document  -->

This is a table with some 📚 and with a `Price` column aligned right:

| Title                   | Author            | Price  |
| ----------------------- | ----------------- | -----: |
| Meditations             | Marcus Aurelius   | $10.00 |
| Rational Optimist       | Matt Ridley       | $12.00 |
| Poor Charlie's Almanack | Charles T. Munger | $16.50 |

This is a math block:

$$$
e=mc^2
$$$

This is a footnote[^1]:

[^1]: The text of the footnote goes here. In the final document the footnote would appear at the bottom of the page.

This is an image:

![md](./Images/md-logo.png)

## Container Blocks 🌳

Here's a list that we've copied from a [GitHub article](https://guides.github.com/features/mastering-markdown/):

- How the Markdown format makes styled collaborative editing easy
- How Markdown differs from traditional formatting approaches
- How to use Markdown to format text
- How to leverage GitHub’s automatic Markdown rendering
- How to apply GitHub’s unique Markdown extensions

This is an ordered list from another GitHub article:

1. **Create a branch**: Topic branches created from the canonical deployment branch (usually master) allow teams to contribute to many parallel efforts. Short-lived topic branches, in particular, keep teams focused and results in quick ships.
2. **Add commits**: Snapshots of development efforts within a branch create safe, revertible points in the project’s history.
3. **Open a pull request**: Pull requests publicize a project’s ongoing efforts and set the tone for a transparent development process.

This is a task list with some random tasks: 

- [x] Write an article about how to manage tasks in a text editor 
- [ ] Repair windshield wipers of car and add washer fluid 🚘
- [ ] Get **2** airplane tickets to ::SF:: for the upcoming [Electron conference](http://electronconf.com/) ✈️

This is a quote:

> And so, my fellow Americans: ask not what your country can do for you — ask what you can do for your country

Oh and since these are container blocks, they can be nested:

> For example, this is a list that's inside of a quote:
> * This is a list item that has been quoted.
> * This is another list item which makes this a list of 2 items.