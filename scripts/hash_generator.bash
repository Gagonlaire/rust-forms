#!/usr/bin/env bash

# create a hash_length that take the first argument or default to 32
hash_length=${1:-32}

node -e "console.log(require('crypto').randomBytes($hash_length).toString('base64'));"
