[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/ddoc.svg
[l1]: https://crates.io/crates/ddoc

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/ddoc/badge.svg
[l3]: https://docs.rs/ddoc/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3


# ddoc

<!-- cradoc start -->


**ddoc** is a markdown based static site generator.

**Complete documentation at [https://dystroy.org/ddoc](https://dystroy.org/ddoc)**

## Usage overview

Create a directory, then move to it

    mkdir website & cd website

Initialize the site:

    ddoc --init

This creates:

- a `.gitignore` file, which eases inclusion of your site in a git managed project
- a `ddoc.hjson` file, holding the basic properties and navigation
- a `src` folder, for your markdown files, CSS style sheets and images

`/src/css/site.css` is a default CSS file, a very simple one which you can remove, or keep as basis for your own incremental changes to get the layout and look you desire.

To build your site, run

    ddoc

This updates a `site` directory, whose content can be sent to your server.

If you want to test it locally, you may run

    ddoc --serve

<!-- cradoc end -->

