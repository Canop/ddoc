
# ddoc

## Introduction

**ddoc** is a markdown based static site generator.

**ddoc** powers the documentations of [broot](https://dystroy.org/broot), [bacon](https://dystroy.org/bacon), [rhit](https://dystroy.org/rhit), [dysk](https://dystroy.org/dysk), [safecloset](https://dystroy.org/safecloset).

**ddoc** makes sense when you want a simple site, such as this one, with a site-wide navigation menu, a table of content on every page, and you want to be confident the style you defined won't be broken at every release of the tool.

**ddoc** aims to have the simplest and fastest installation and configuration process, after which you just have to add markdown files and edit your CSS in a standard and obvious way.

## Project Goals, and features

The development of ddoc was motivated by the frequent breakages occuring with other popular documentation generators.

You should be able to generate your site from whatever system, whatever version of the tool, and get the same style.

You should also not have to deal with huge imported theming CSS files full of `!important` and crumbling under their complexity.

Summarizing the goals and features of **ddoc**:

* A reliable static site generator for documentation sites
* Complete and reasonable navigation (menu, TOC, links, search)
* Avoid breaks among versions - no imported CSS
* Support images, tables, code sections, links, etc.
* Cross-platform and easy to install - a single binary with no dependencies
* Clean URL paths, no history shenanigans, obvious links
* Generated HTML is easy to style with CSS
* All internal links are relative, ddoc doesn't need to know the base url of the site
* Integrated search function - but you can replace it with your own

## Why NOT use ddoc

* ddoc assumes you want to write or tune, then own, your CSS, not choose among themes
* ddoc has no templating system - it doesn't suit every need
* ddoc has no plugin system
* ddoc is very new and might need fixes according to feedback

## Getting Started

* [Install ddoc](install)
* [Setup your site](setup)
* [Edit your site](edit)
* [Look at examples](examples)

