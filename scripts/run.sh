#!/bin/bash

cd "$(dirname "$0")" || exit
./build.sh || exit
../target/debug/cinnamon-desktop-editor $@
