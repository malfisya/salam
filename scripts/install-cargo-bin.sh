#!/bin/sh
# SPDX-FileCopyrightText: Copyright © 2025 Muhammad Alfi Syahrin
#
# SPDX-License-Identifier: MPL-2.0

# Usage: install-cargo-bin.sh <project-root> <binary-name> <profile>

proj_root="$1"
bin="$2"
profile="$3"

src="$proj_root/target/$profile/$bin"
dest="$DESTDIR$MESON_INSTALL_PREFIX/bin/$bin"

echo "Installing $src -> $dest"
install -Dm755 "$src" "$dest"
