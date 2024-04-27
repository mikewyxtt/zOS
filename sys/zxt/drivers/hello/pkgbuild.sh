#!/usr/bin/env bash
PKGNAME="hello"
VERSION="0.0.0"

build() {
    echo Building $PKGNAME $VERSION

    cargo build --verbose --target=$TARGET_ARCH
}

clean() {
    cargo clean
}

build_install() {
    echo
    cp target/x86_64-unknown-none/debug/hello $1/hello.zxt
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"