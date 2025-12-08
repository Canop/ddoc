
# Create Pages

Each page is backed by a markdown file in the `/src` directory.

You may organize your files in subdirectories but you don't have to, the tree of files doesn't have to match the menu tree.

Each file, to be served, must be referenced in the `pages` section of the `ddoc.hjson` config file.

Standard features of CommonMark are available, including links, images, tables, etc.

# Change the style

You don't want to keep the neutral grey look of the default CSS.

The easiest solution is to modify `src/css/site.css` to your taste, either by modifying the variables defined on top, by changing the existing rules or by adding more selective ones.

As long as there's at least one file in `src/css`, `ddoc --init` won't add anything in there, nor replace the `site.css` file, so it's perfectly fine to change it.

Of course you may also start from scratch with a brand new CSS file.

# Add CSS files and JS files

Any file matching `/src/css/*.css` or `/src/js/*.js` will be served.

Files are included in the HTML's head element in alphabetic order.

The body element holds a page-specific class to help discriminate rules.
For example, on this site the `<h1>` elements of the [examples](../examples) page have a top border, which is defined with a rule on `body.page-examples h1`.

# Include images

Images have to be stored in `/src/img`.

They can be included  in markdown using standard syntax:

```markdown
![Alt text](img/my_image.png)
```

You'll also have references to images in your ddoc.hjson config file, for example in menu
links.

In both cases, ddoc will rewrite the image URL to point to the correct location whatever
the depth of the page including the image. Your reference just has to start with `img/`.

# Internal links

## Links to pages

In your markdown or in the ddoc.hjson config file, you can refer to other pages, or locations
in other pages, using either relative or absolute links.

A relative link is like `../other_page#some_section`.

An absolute link is like `/path/to/other_page#some_section`.

The leading `/` in absolute links refers to the root of your documentation site and the URL
will be rewritten to be relative to the current page depending on its depth.

## Expansions

Some special values are dynamically expanded:

- `--previous-page` links to the previous page according to the order defined by the pages list
- `--next-page` links to the next page
- `--search` opens the search dialog (and its presence triggers the inclusion of a search script)

## Javascript call

There's no problem calling a javascript function of one of your scripts from a link.

For example, you can have this in your ddoc.hjson:

```Hjson
header: {
    left: menu
    right: [
        {
            label: ping
            href: "javascript:alert('test');"
        }
    ]
}
```
