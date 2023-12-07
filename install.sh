#!/bin/bash
##  if service exists stop it
if [[ -f /etc/systemd/system/gitspore.service ]]; then systemctl stop gitspore; else echo "na"; fi


## erase install folder and recreate it
rm -rf /var/lib/gitspore/
mkdir /var/lib/gitspore/
SERVICE_USER=$(cat config/Service.toml | grep service_user | cut -d ' ' -f 3 | sed 's/\"//g')
chown -R "$SERVICE_USER":"$SERVICE_USER" /var/lib/gitspore/

## run bulid
cargo build --release

## copy binary and host_list.txt to install dir
cp target/release/gitspore /var/lib/gitspore/gitspore
cp -r run.sh /var/lib/gitspore/


## clean build to free up space taken
#cargo clean

## copy service file and reload systemd daemon
cp  gitspore.service /etc/systemd/system/gitspore.service
systemctl daemon-reload

## enable and start service
systemctl enable gitspore
systemctl start gitspore
