#!/usr/bin/env bash
PKGNAME="initializer"
VERSION="0.0.0"

build() {
    echo Building $PKGNAME $VERSION

    cargo build --verbose --target=i686.json --features serial_debug
}

clean() {
    cargo clean
}

install() {
    cp target/i686/debug/initializer $CHIMERA_SRC_ROOT/iso/boot/initializer
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"