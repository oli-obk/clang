#![feature(plugin_registrar, rustc_private, plugin, slice_patterns)]

extern crate syntax;
extern crate rustc;
extern crate clang;

use clang::*;

use std::ffi::{CString, CStr};
use std::fmt::Write;
use std::mem::transmute;
use std::str::from_utf8;

use syntax::codemap::{Span, BytePos, Spanned};
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken, TtDelimited, TtSequence, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc::plugin::Registry;

fn expand_include_cpp(
    cx: &mut ExtCtxt,
    sp: Span,
    args: &[TokenTree]
    ) -> Box<MacResult + 'static>
{
    let (text, sp) = match args {
        [TtToken(sp, token::Literal(token::Lit::Str_(s), _))] => (s, sp),
        [..] => {
            cx.span_err(sp, "expected a single string literal");
            return DummyResult::any(sp);
        }
    };
    let text = text.as_str();
    let idx = unsafe { clang_createIndex(1, 1) };
    let filename = CString::new(text).unwrap();
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
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(0usize)) });
    unimplemented!()
}

#[allow(unconditional_recursion)]
extern fn cb(cursor: CXCursor, _parent: CXCursor, _client_data: CXClientData) -> CXChildVisitResult
{
    let t = unsafe { clang_getCursorType(cursor.clone()) };
    let name = unsafe { clang_Cursor_getMangling(cursor.clone()) };
    let c_name = unsafe { from_utf8(CStr::from_ptr(clang_getCString(name)).to_bytes()).unwrap() };
    println!("{:?}: {:?} {:?}", cursor.kind, t.kind, c_name);
    CXChildVisitResult::CXChildVisit_Continue
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("include_cpp", expand_include_cpp);
}
