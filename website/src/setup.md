
# Initialize a whole new site

Create a directory, then move to it

    mkdir website & cd website

"website" is a reasonnable name for the documentation part of a project.

Initialize the site:

    ddoc --init

This creates:

- a `.gitignore` file, which eases inclusion of your site in a git managed project
- a `ddoc.hjson` file, holding the basic properties and navigation
- a `src` folder, for your markdown files, CSS style sheets and images

`/src/css/site.css` is a default CSS file, a very simple one which you can remove, or keep as basis for your own incremental changes to get the layout and look you desire.

Sometimes, the content of the initial `src/index.md` file, or some properties of the `ddoc.hjson` config will be guessed from the parent directory.

# Build the site

To build your site, run

    ddoc

This updates a `site` directory, whose content can be sent to your server.

If you want to test it locally, you may run

    ddoc --serve

**Note:** The current version of ddoc doesn't rebuild the site on file changes. So `ddoc --serve` is best used combined with a watcher, eg [bacon](https://dystroy.org/bacon).

Now that you've see the initial, quite void, site, you should [edit it](../edit).

# Restore some defaults

You won't break anything if you run again `ddoc --init`.

If you already have your `src` directory full of markdown files, ddoc will add what's missing.

If you don't have a `ddoc.hjson` file, it will be created.

If you don't have a `src/index.md` file, one will be written.

If you don't have a `src/css` directory, the default `src/css/site.css` file will be written.

If nothing is obviously missing, ddoc won't do anything.

So to restore defaults, remove some part and run `ddoc --init`.

