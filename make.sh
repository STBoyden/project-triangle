#!/bin/bash

DIR="$(dirname "$0")"
DIR_NAME="$(basename "$(pwd)")"

RSRC_DIR="assets"
DSTRB_DIR="distrib"

[ -d "$DIR/target/debug" ] && rm -rf "$DIR/target/debug/$RSRC_DIR"
[ -d "$DIR/target/release" ] && rm -rf "$DIR/target/release/$RSRC_DIR"

IFS=' ' read -ra commands <<<"$@"

if cargo "$@"; then
  [ -d "$DIR/target/debug" ] && cp -r "$DIR/$RSRC_DIR" "$DIR/target/debug/$RSRC_DIR"
  [ -d "$DIR/target/release" ] && cp -r "$DIR/$RSRC_DIR" "$DIR/target/release/$RSRC_DIR"
fi

[ ! -d "$DIR/$DSTRB_DIR" ] && mkdir "$DIR/$DSTRB_DIR"

for i in "${commands[@]}"; do
  if [[ "$i" == "--release" || "$i" == "release" ]]; then
    temp_distrib="$(pwd)/distrib-temp"
    original_dir="$(pwd)"

    [ ! -d "$temp_distrib" ] && mkdir "$temp_distrib"
    cp "$DIR/target/release/$DIR_NAME" "$temp_distrib"
    cp "$DIR/target/release/$DIR_NAME.exe" "$temp_distrib"
    cp -r "$DIR/target/release/$RSRC_DIR" "$temp_distrib/"

    cd "$temp_distrib" || exit

    echo ""
    echo "--- [ Creating Linux release zip ] ---"
    zip -r "${DIR_NAME}_linux-x86_64_$(date --iso-8601).zip" \
      "$DIR_NAME" \
      "$RSRC_DIR"

    echo ""
    echo "--- [ Creating Windows release zip ] ---"
    zip -r "${DIR_NAME}_windows-x86_64_$(date --iso-8601).zip" \
      "$DIR_NAME.exe" \
      "$RSRC_DIR"

    echo ""
    cd "$original_dir" || exit

    for f in "$temp_distrib"/*; do
      if [[ "${f##*.}" == "zip" ]]; then
        cp "$f" "$DSTRB_DIR"
      fi
    done

    rm -rf "$temp_distrib"
    break
  fi
done
