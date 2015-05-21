extern crate clang;

use clang::*;
use std::ffi::CString;

#[test]
fn parse_header() {
    let idx = unsafe { clang_createIndex(1, 1) };
    let filename = CString::new("tests/cpp_functions.hpp").unwrap();
    let tu = unsafe { clang_parseTranslationUnit(
        idx,
        filename.as_ptr(),
        std::ptr::null(),
        0,
        std::ptr::null(),
        0,
        0,
    ) };
    assert!(tu as *const CXTranslationUnitImpl != std::ptr::null());
}
