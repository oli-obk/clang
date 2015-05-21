extern crate clang;

use clang::*;

#[test]
fn create_index() {
    let _ = clang_createIndex(1, 1);
}
