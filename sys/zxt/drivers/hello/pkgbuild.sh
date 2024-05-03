#!/usr/bin/env bash
PKGNAME="hello"
VERSION="0.0.0"

do_build() {
    echo Building $PKGNAME $VERSION

    cargo build --verbose --target=$TARGET_ARCH
}

do_clean() {
    cargo clean
}

build_do_install() {
    echo
    cp target/x86_64-unknown-none/debug/hello $1/hello.zxt
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"