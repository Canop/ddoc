
# ddoc

## Introduction

**ddoc** is a markdown based static site generator.

**ddoc** powers the documentations of [broot](https://dystroy.org/broot), [bacon](https://dystroy.org/bacon), [rhit](https://dystroy.org/rhit), [dysk](https://dystroy.org/dysk), [safecloset](https://dystroy.org/safecloset).

**ddoc** is less powerful than other tools (Hugo, Zola, Mkdocs, etc.) and doesn't include templating or plugins systems.

**ddoc** makes sense when you want a simple site, such as this one, with a site-wide navigation menu, a table of content on every page, and you want to be confident the style you defined won't be broken at every release of the tool.

**ddoc** aims to have the simplest and fastest installation and configuration process, after which you just have to add markdown files and edit your CSS in a standard and obvious way.

## Why NOT use ddoc

* ddoc assumes you want to write or tune, then own, your CSS, not choose among themes
* ddoc has no templating system - it doesn't suit every need
* ddoc has fewer features than other static site generators
* ddoc is very new and might need fixes according to feedback

## Project Goals, and features

* Be a reliable static site generator for documentation sites
* Complete and reasonable navigation (menu, TOC, links, search)
* Avoid breaks among versions - no imported CSS
* Support images, tables, code sections, links, etc.
* Cross-platform and easy to install - a single binary with no dependencies
* Clean URL paths, no history shenanigans, obvious links
* Generated HTML is semantic and easy to style with CSS
* All internal links are relative, ddoc doesn't need to know the base url of the site

## Possible future goals

* Automated lists and page types, to make ddoc suitable for blogs, albums, etc.
* Image processing
* Syntax highlighting in code

Those goals are pre-designed for retro-compatibility, to ensure ddoc sites aren't broken by new versions.

## Getting Started

* [Install ddoc](install)
* [Setup your site](setup)
* [Edit your site](edit)
* [Look at examples](examples)

