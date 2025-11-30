# This script is dedicated to the official documentation site at https://dystroy.org/ddpc

# build the documentation site
ddoc

# copy the site to the deployement stage
cp -r site/* ~/dev/www/dystroy/ddoc/

# deploy on dystroy.org
~/dev/www/dystroy/deploy.sh
