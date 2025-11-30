

# ddoc

**Warning: ddoc is very (very very) recent and still considered unstable**

## Introduction

**ddoc** is a markdown based static site generator.

**ddoc** is *much* less powerful than other tools (Hugo, Zola, Mkdocs, etc.) and doesn't include templating or plugins systems.

**ddoc** makes sense when you want a simple site, such as this one, with a site wide navigation menu, a table of content on every page, and you want to be confident the style won't be broken at every release of the tool.


## Why NOT use ddoc

* ddoc assumes you want to write or tune, then own, your CSS, not choose among themes
* ddoc has less features than any other static site generator
* this tool is super very new and not battle tested

## Project Goals

* Be a reliable static site generator for documentation sites
* Complete and reasonnable navigation (menu, TOC, links)
* Avoid breaks among versions - no imported CSS or layout related HTML
* Support images, tables, code sections, links, etc.
* Cross-platform and easy to install - a single binary with no dependencies
* Clean URL paths, no history shenanigans, obvious links
* Work without JS (but you can add your own JS if you want to)

## Project Non Goals

* Be as generic as zola, mkdocs, hugo, etc. and try to replace them
* Templating - **ddoc probably can't do what you need**
* Theming system - you provide your own CSS

## Possible future goals

* Search
* Automated "list" pages - to make ddoc suitable for blogs, albums, etc.
* Image processing
* Syntax highlighting in code

## Features

* Generated HTML is semantic and easy to style with CSS
* All internal links are relative, ddoc doesn't need to know the base url of the site
* No hidden CSS or JS is injected, only yours
* No templating - everything is built from your markdown, static files, and the ddoc.hjson config

## Getting Started

* [Install ddoc](install)
* [Setup your site](setup)
* [Edit your site](edit)



