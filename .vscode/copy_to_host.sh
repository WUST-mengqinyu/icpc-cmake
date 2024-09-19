#!/usr/bin/env bash
set -ex
curl http://127.0.0.1:10045/proxy/send_clipboard?file_path=$1
# echo "$1" | nc localhost 10045