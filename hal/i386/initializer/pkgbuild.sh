PKGNAME="initializer"
VERSION="0.0.0"

build() {
    echo Building $PKGNAME $VERSION

    make -j8
    #RUSTFLAGS="-C link-arg=start.o -C link-arg=-Tlink.ld" cargo build --verbose --target=i686.json --features serial_debug
    cargo build --verbose --target=i686.json --features serial_debug
    #RUSTFLAGS="--extern start=start.o" cargo build --verbose --target=i686.json
}

clean() {
    make clean
    cargo clean
}

install() {
    cp target/i686/debug/initializer $CHIMERA_SRC_ROOT/iso/boot/initializer
}

# cd to directory of this package, then execute the function specified as an argument
cd "${0%/*}"
"$@"