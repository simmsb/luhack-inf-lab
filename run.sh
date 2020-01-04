#!/bin/sh

cd "$1" || exit

ARGS=$(grep -E -v '^#' env | xargs printf -- '--build-arg %s\n' | xargs)

echo "$ARGS"

/usr/bin/env docker build $ARGS -t "$1" .
/usr/bin/env container-per-ip "$1" -p $2
