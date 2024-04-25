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

install() {
    echo
    #cp target/i686/debug/initializer $zOS_SRC_ROOT/iso/boot/initializer
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"