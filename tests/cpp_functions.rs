extern crate clang;

use clang::*;
use std::ffi::CString;

use std::mem::transmute;

struct MyData {
    depth: i32,
}

#[allow(unconditional_recursion)]
extern fn cb(cursor: CXCursor, parent: CXCursor, client_data: CXClientData) -> CXChildVisitResult
{
    let myData: &mut MyData = unsafe{ transmute(client_data) };
    for _ in 0..myData.depth*4 {
        print!(" ");
    }
    println!("{:?}", cursor.kind);
    let mut innerData = MyData {
        depth: myData.depth + 1,
    };
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut innerData)) });
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
    let mut myData = MyData {
        depth: 0,
    };
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut myData)) });
    unimplemented!()
}
