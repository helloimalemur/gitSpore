#!/bin/bash
tar cvzf "/mnt/1TB/backups/git_archive/git-archive-$(date '+%Y%m%d').tar.gz" /mnt/1TB/backups/git/
rm -rf /mnt/1TB/backups/git/*
cd /var/lib/gitspore/ && ./gitspore
