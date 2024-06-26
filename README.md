zOS is a UNIX-like operating system.

Notable design choices:
	- Modular design; similar to a microkernel but all servers and drivers operate inside a shared address space to avoid performance consequences associated with message passing. Rust's protection mechanisms are good enough to ensure stability. 
	- Networking Kernel, message passing utilizes packet based system similar to TCP/IP
	- Hardware Abstraction Layer to ensure easy portability

# Running in qemu
```sh
qemu-system-x86_64  -bios <path-to-ovmf> \
					-drive if=none,id=stick,format=raw,file=RELEASE/<zOS Release IMG> \
					-device nec-usb-xhci,id=xhci \
					-device usb-storage,bus=xhci.0,drive=stick
```


# Setting up Rust
```sh
rustup component add rust-src
rustup target add x86_64-unknown-uefi
rustup target add x86_64-unknown-none


```

# Required packages to build:
clang, rust

# Required packages to create iso
xorriso, mtools, fdisk

mkdosfs, mkfs.ext2