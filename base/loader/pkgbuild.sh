#!/usr/bin/env bash
PKGNAME="loader"
VERSION="0.0.0"

build() {
    echo Building $PKGNAME $VERSION

    cargo build --verbose --target=x86_64-unknown-uefi
}

clean() {
    cargo clean
}

build_install() {
    return
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"