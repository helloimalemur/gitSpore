#!/bin/bash
tar cvzf "/mnt/4TB/backups/git_archive/git-archive-$(date '+%Y%m%d').tar.gz" /mnt/4TB/backups/git/
rm -rf /mnt/4TB/backups/git/*
cd /var/lib/gitspore/ && ./gitspore
