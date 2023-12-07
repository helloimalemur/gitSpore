#!/bin/bash


## erase install folder and recreate it
rm -rf /var/lib/gitspore/
mkdir /var/lib/gitspore/

## run bulid
cargo build --release

## copy binary and host_list.txt to install dir
cp target/release/gitspore /var/lib/gitspore/gitspore
cp -r config/ /var/lib/gitspore/
if [[ -f /root/.gitspore_settings ]]; then cp /root/.gitspore_settings /var/lib/gitspore/config/Settings.toml; else echo "na"; fi
cp -r run.sh /var/lib/gitspore/

## clean build to free up space taken
#cargo clean
