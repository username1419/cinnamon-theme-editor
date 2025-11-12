#!/bin/bash

original_path=$(pwd)

cd "$(dirname "$0")" || exit
cd ../

# setup
glib-compile-resources data/resources.gresource.xml --target=data/resources.gresource --sourcedir=data || exit

# build
cargo test $@

# cleanup
echo "dont edit this file its just to stop the compiler from complaining" >data/resources.gresource

cd $original_path
