#!/usr/bin/env bash
PKGNAME="loader"
VERSION="0.0.0"

do_build() {
    echo Building $PKGNAME $VERSION

    cargo build --verbose --target=x86_64-unknown-uefi
}

do_clean() {
    cargo clean
}

do_install() {
    return
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"