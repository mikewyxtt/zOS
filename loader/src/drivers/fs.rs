// file.rs

#![allow(dead_code)]

pub struct File {

}

pub fn open(path: &str) -> File {
    if path.starts_with("/boot/efi") {

    }
    File {}
}