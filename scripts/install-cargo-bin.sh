#!/bin/sh
# Usage: install-cargo-bin.sh <binary-name> <profile>

bin="$1"
profile="$2"

src="target/$profile/$bin"
dest="$DESTDIR$MESON_INSTALL_PREFIX/bin/$bin"

echo "Installing $src -> $dest"
install -Dm755 "$src" "$dest"
