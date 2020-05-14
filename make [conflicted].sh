#!/bin/bash

DIR="$(dirname "$0")"
DIR_NAME="$(basename "$(pwd)")"

RSRC_DIR="assets"
DSTRB_DIR="distrib"


[ -d "$DIR/target/debug" ] && rm -rf "$DIR/target/debug/$RSRC_DIR"
[ -d "$DIR/target/release" ] && rm -rf "$DIR/target/release/$RSRC_DIR"

IFS=' ' read -ra commands <<< "$@"


if cargo "$@"; then
    [ -d "$DIR/target/debug" ] && cp -r "$DIR/$RSRC_DIR" "$DIR/target/debug/$RSRC_DIR"
    [ -d "$DIR/target/release" ] && cp -r "$DIR/$RSRC_DIR" "$DIR/target/release/$RSRC_DIR"
fi

[ ! -d "$DIR/$DSTRB_DIR" ] && mkdir "$DIR/$DSTRB_DIR"

for i in "${commands[@]}"; do
  if [[ "$i" == "--release" ]]; then
    [ -d "$DIR/target/release" ] && zip -r "$DIR/$DSTRB_DIR/${DIR_NAME}_$(date --iso-8601).zip" "$DIR/target/release/$DIR_NAME" "$DIR/target/release/$RSRC_DIR"
  fi
done
