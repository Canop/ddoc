
# Initialize a whole new site

Create a directory, then move to it

    mkdir website & cd website

Initialize the site:

    ddoc --init

This creates:

- a `.gitignore` file, which eases inclusion of your site in a git managed project
- a `ddoc.hjson` file, holding the basic properties and navigation
- a `src` folder, for your markdown files, CSS style sheets and images

`/src/css/site.css` is a default CSS file, a very simple one which you can remove, or keep as basis for your own incremental changes to get the layout and look you desire.

**ddoc** tries to guess relevant properties (eg the name of the site) from the parent directory, in order to fill the initial `src/index.md` file and the `ddoc.hjson` config file.

# Build the site

To build your site, run

    ddoc

This updates a `site` directory, whose content can be sent to your server.

If you want to test it locally, you may run

    ddoc --serve

**ddoc** rebuilds the site on changes but the page isn't automatically reloaded (the site is served without injected script, exactly as you'll use it in production), so you'll have to refresh the page in the browser to see the change.

Now that you've see the initial, quite void, site, you should [edit it](../edit).

# Restore some defaults

You won't break anything if you run again `ddoc --init`.

If you already have your `src` directory full of markdown files, ddoc will add what's missing.

If you don't have a `ddoc.hjson` file, it will be created.

If you don't have a `src/index.md` file, one will be written.

If you don't have a `src/css` directory, the default `src/css/site.css` file will be written.

If nothing is obviously missing, ddoc won't do anything.

So to restore defaults, remove some part and run `ddoc --init`.

