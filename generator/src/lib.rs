#![feature(plugin_registrar, rustc_private, plugin, slice_patterns)]

extern crate syntax;
extern crate rustc;
extern crate clang;

use clang::*;

use std::ffi::{CString, CStr};
use std::fmt::Write;
use std::mem::transmute;
use std::str::from_utf8;

use syntax::codemap::{Span, BytePos, Spanned, mk_sp};
use syntax::parse::token::{self, intern};
use syntax::ast::{TokenTree, TtToken, TtDelimited, TtSequence, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::util::small_vector::SmallVector;
use syntax::ptr::P;
use syntax::ast::{Item, StructDef};
use rustc::plugin::Registry;

struct Helper<'cx, 'a: 'cx> {
    cx: &'cx mut ExtCtxt<'a>,
    v: SmallVector<P<Item>>,
}

fn expand_include_cpp<'cx, 'a>(
    cx: &'cx mut ExtCtxt<'a>,
    sp: Span,
    args: &[TokenTree]
    ) -> Box<MacResult + 'static>
{
    let (filename, sp) = match args {
        [TtToken(sp, token::Literal(token::Lit::Str_(s), _))] => (s, sp),
        [..] => {
            cx.span_err(sp, "expected a single string literal");
            return DummyResult::any(sp);
        }
    };
    let filename = filename.as_str();
    let idx = unsafe { clang_createIndex(1, 1) };
    let filename = CString::new(filename).unwrap();
    let args: Vec<&'static str> = vec!["-I", "."];
    let args: Vec<_> = args.iter().map(|&s| CString::new(s).unwrap()).collect();
    let args: Vec<_> = args.iter().map(|s| s.as_ptr()).collect();
    let tu = unsafe { clang_parseTranslationUnit(
        idx,
        filename.as_ptr(),
        args.as_ptr(),
        args.len() as i32,
        std::ptr::null(),
        0,
        0,
    ) };
    assert!(tu as *const CXTranslationUnitImpl != std::ptr::null());
    let cursor = unsafe { clang_getTranslationUnitCursor(tu) };
    let mut helper = Helper {
        v: SmallVector::zero(),
        cx: cx,
    };
    assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut helper)) });
    MacEager::items(helper.v)
}

fn parse_struct(cx: &mut ExtCtxt, name: &str, sp: Span, cursor: CXCursor) -> P<Item> {
    let ident = Ident::new(intern(name));
    let fields = vec![];
    let struct_def = StructDef {
        fields: fields,
        ctor_id: None, // FIXME
    };
    cx.item_struct(sp, ident, struct_def)
}

struct TypedefHelper<'cx, 'a: 'cx> {
    cx: &'cx mut ExtCtxt<'a>,
    sp: Span,
    name: &'a str,
    item: Option<P<Item>>,
}

#[allow(unconditional_recursion)]
extern fn cb_typedef(
    cursor: CXCursor,
    _parent: CXCursor,
    client_data: CXClientData,
    ) -> CXChildVisitResult
{
    use clang::CXCursorKind::*;
    let &mut TypedefHelper {
        ref mut cx,
        ref name,
        sp,
        ref mut item,
    } = unsafe { transmute(client_data) };
    match cursor.kind {
        CXCursor_StructDecl => *item = Some(parse_struct(cx, name, sp, cursor)),
        _ => unimplemented!(),
    }
    CXChildVisitResult::CXChildVisit_Continue
}

#[allow(unconditional_recursion)]
extern fn cb(cursor: CXCursor, _parent: CXCursor, client_data: CXClientData) -> CXChildVisitResult
{
    use clang::CXCursorKind::*;
    let client_data: &mut Helper = unsafe { transmute(client_data) };
    let t = unsafe { clang_getCursorType(cursor.clone()) };
    let mangled = unsafe { clang_Cursor_getMangling(cursor.clone()) };
    let mangled = unsafe { from_utf8(CStr::from_ptr(clang_getCString(mangled)).to_bytes()).unwrap() };
    let name = unsafe { clang_getCursorSpelling(cursor.clone()) };
    let name = unsafe { from_utf8(CStr::from_ptr(clang_getCString(name)).to_bytes()).unwrap() };
    println!("{:?}: {:?} {:?} {:?}", cursor.kind, t.kind, mangled, name);
    let sp = mk_sp(BytePos(0), BytePos(0));
    let item = match cursor.kind {
        CXCursor_StructDecl => {
            // typedef struct {.. } <Name>
            if name == "" { return CXChildVisitResult::CXChildVisit_Continue; }
            parse_struct(client_data.cx, name, sp, cursor)
        }
        CXCursor_TypedefDecl => {
            assert!(name != "");
            let mut help = TypedefHelper {
                cx: client_data.cx,
                sp: sp,
                name: name,
                item: None,
            };
            assert_eq!(0, unsafe { clang_visitChildren(cursor, cb_typedef, transmute(&mut help)) });
            help.item.unwrap()
        }
        CXCursor_FunctionDecl if mangled.ends_with(name) => {
            // c-function b/c mangled name ends with unmangled name
            // FIXME: find better method of detection
        }
        _ => unimplemented!(),
    };
    client_data.v.push(item);
    CXChildVisitResult::CXChildVisit_Continue
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("include_cpp", expand_include_cpp);
}
