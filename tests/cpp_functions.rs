extern crate clang;

use clang::*;
use std::ffi::{CString, CStr};

use std::mem::transmute;
use std::str::from_utf8;

struct MyData {
    depth: i32,
}

#[allow(unconditional_recursion)]
extern fn cb(cursor: CXCursor, parent: CXCursor, client_data: CXClientData) -> CXChildVisitResult
{
    let my_data: &mut MyData = unsafe{ transmute(client_data) };
    for _ in 0..my_data.depth*4 {
        print!(" ");
    }
    let t = unsafe { clang_getCursorType(cursor.clone()) };
    let name = unsafe { clang_Cursor_getMangling(cursor.clone()) };
    let c_name = unsafe { from_utf8(CStr::from_ptr(clang_getCString(name)).to_bytes()).unwrap() };
    println!("{:?}: {:?} {:?}", cursor.kind, t.kind, c_name);
    let mut inner_data = MyData {
        depth: my_data.depth + 1,
    };
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut inner_data)) });
    CXChildVisitResult::CXChildVisit_Continue
}

#[test]
fn parse_header() {
    println!("");
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
    let mut my_data = MyData {
        depth: 0,
    };
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut my_data)) });
    unimplemented!()
}
