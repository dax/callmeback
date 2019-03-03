#!/bin/sh

[ -x ./bin/diesel ] || cargo install diesel_cli --no-default-features --features postgres --root .
./bin/diesel setup --database-url "$POSTGRESQL_ADDON_URI"
