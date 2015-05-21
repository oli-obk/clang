extern crate clang;

use clang::*;

#[test]
fn create_index() {
    let _ = unsafe { clang_createIndex(1, 1) };
}
