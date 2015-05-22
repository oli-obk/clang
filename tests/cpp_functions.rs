extern crate clang;
extern crate libc;

use clang::*;
use std::ffi::CString;
use libc::c_void;

extern fn cb(cursor: CXCursor, parent: CXCursor, client_data: CXClientData) -> CXChildVisitResult
{
    println!("I am a callback");
    CXChildVisitResult::CXChildVisit_Recurse
}

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
    let cursor = unsafe { clang_getTranslationUnitCursor(tu) };
    let visitor = unsafe { clang_visitChildren(cursor, cb, 0 as *mut c_void) };
}
