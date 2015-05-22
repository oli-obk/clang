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
use syntax::parse::token::{self, intern, InternedString};
use syntax::ast::{TokenTree, TtToken, TtDelimited, TtSequence, Ident, Arg, Ty, FnDecl};
use syntax::ast::{ForeignItem, LitStr};
use syntax::ast::Mutability::*;
use syntax::{ast, abi, ast_util};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::util::small_vector::SmallVector;
use syntax::ptr::P;
use syntax::ast::{Item, StructDef};
use rustc::plugin::Registry;

struct Helper<'cx, 'a: 'cx> {
    cx: &'cx mut ExtCtxt<'a>,
    v: SmallVector<P<Item>>,
    fns: Vec<P<ForeignItem>>,
    sp: Span,
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
    let libc = cx.item(
        sp,
        Ident::new(intern("libc")),
        vec![],
        ast::ItemExternCrate(None),
    );
    let (mut v, fns) = {
        let mut helper = Helper {
            v: SmallVector::one(libc),
            cx: cx,
            fns: vec![],
            sp: sp,
        };
        assert_eq!(0, unsafe { clang_visitChildren(cursor, cb, transmute(&mut helper)) });
        (helper.v, helper.fns)
    };
    let externs = ast::ForeignMod {
        abi: abi::Abi::C,
        items: fns,
    };
    v.push(cx.item(
        sp,
        Ident::new(intern("something_extern_something")),
        vec![],
        ast::ItemForeignMod(externs),
    ));
    MacEager::items(v)
}

struct FunargHelper<'cx, 'a: 'cx> {
    cx: &'cx mut ExtCtxt<'a>,
    sp: Span,
    args: Vec<Arg>,
}

#[allow(unconditional_recursion)]
extern fn cb_funargs(
    cursor: CXCursor,
    _parent: CXCursor,
    client_data: CXClientData,
    ) -> CXChildVisitResult
{
    use clang::CXCursorKind::*;
    let &mut FunargHelper {
        ref mut cx,
        sp,
        ref mut args,
    } = unsafe { transmute(client_data) };
    args.push(parse_arg(cx, sp, cursor));
    CXChildVisitResult::CXChildVisit_Continue
}

fn parse_ty(cx: &mut ExtCtxt, sp: Span, ty: CXType) -> P<Ty> {
    use clang::CXTypeKind::*;
    println!("{:?}", ty.kind);
    let name = unsafe { clang_getTypeSpelling(ty.clone()) };
    let name = unsafe { from_utf8(CStr::from_ptr(clang_getCString(name)).to_bytes()).unwrap() };
    println!("name: {:?}", name);
    match ty.kind {
        CXType_Typedef => {
            let ident = Ident::new(intern(name));
            cx.ty_ident(sp, ident)
        }
        CXType_Pointer => {
            let inner = unsafe { clang_getPointeeType(ty.clone()) };
            let inner = parse_ty(cx, sp, inner);
            let muta = if unsafe { clang_isConstQualifiedType(ty.clone()) } == 1 {
                MutImmutable
            } else {
                MutMutable
            };
            cx.ty_ptr(sp, inner, muta)
        }
        CXType_Char_S => {
            let path = vec!["libc", "c_char"];
            let path = path.iter().map(|p| Ident::new(intern(p))).collect();
            let path = cx.path(sp, path);
            cx.ty_path(path)
        }
        _ => unimplemented!(),
    }
}

fn parse_arg(cx: &mut ExtCtxt, sp: Span, cursor: CXCursor) -> Arg {
    let name = unsafe { clang_getCursorSpelling(cursor.clone()) };
    let name = unsafe { from_utf8(CStr::from_ptr(clang_getCString(name)).to_bytes()).unwrap() };
    println!("funarg: {:?}", name);
    let ident = Ident::new(intern(name));
    let ty = parse_ty(cx, sp, unsafe { clang_getCursorType(cursor) });
    cx.arg(sp, ident, ty)
}

fn parse_fn(cx: &mut ExtCtxt, name: &str, sp: Span, cursor: CXCursor) -> P<ForeignItem> {
    use clang::CXTypeKind::*;
    let t = unsafe { clang_getCursorType(cursor.clone()) };
    let ret = unsafe { clang_getResultType(t.clone()) };
    let args = {
        let mut help = FunargHelper {
            cx: cx,
            sp: sp,
            args: vec![],
        };
        assert_eq!(0, unsafe { clang_visitChildren(cursor, cb_funargs, transmute(&mut help)) });
        help.args
    };
    let ret = match ret.kind {
        CXType_Void => ast::NoReturn(sp),
        _ => ast::Return(parse_ty(cx, sp, ret)),
    };
    let f = P(ast::FnDecl {
        inputs: args,
        output: ret,
        variadic: false
    });
    P(ForeignItem {
        ident: intern(name).ident(),
        attrs: vec![],
        id: ast::DUMMY_NODE_ID, // FIXME
        span: sp,
        vis: ast::Visibility::Public,
        node: ast::ForeignItemFn(f, ast_util::empty_generics()),
    })
}

fn parse_struct(cx: &mut ExtCtxt, name: &str, sp: Span, cursor: CXCursor) -> P<Item> {
    let ident = Ident::new(intern(name));
    let fields = vec![];
    let struct_def = StructDef {
        fields: fields,
        ctor_id: None, // FIXME
    };
    let s = cx.item_struct(sp, ident, struct_def);
    let c = cx.meta_word(sp, InternedString::new("C"));
    let meta = cx.meta_list(
        sp,
        InternedString::new("repr"),
        vec![c],
    );
    s.map(|mut s| {
        s.attrs.push(cx.attribute(sp, meta));
        s
    })
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
    match cursor.kind {
        CXCursor_StructDecl => {
            // typedef struct {.. } <Name>
            if name == "" { return CXChildVisitResult::CXChildVisit_Continue; }
            client_data.v.push(parse_struct(client_data.cx, name, client_data.sp, cursor));
        }
        CXCursor_TypedefDecl => {
            assert!(name != "");
            let mut help = TypedefHelper {
                cx: client_data.cx,
                sp: client_data.sp,
                name: name,
                item: None,
            };
            assert_eq!(0, unsafe { clang_visitChildren(cursor, cb_typedef, transmute(&mut help)) });
            client_data.v.push(help.item.unwrap());
        }
        CXCursor_FunctionDecl if mangled.ends_with(name) => {
            // c-function b/c mangled name ends with unmangled name
            // FIXME: find better method of detection
            client_data.fns.push(parse_fn(client_data.cx, name, client_data.sp, cursor));
        }
        _ => unimplemented!(),
    };
    CXChildVisitResult::CXChildVisit_Continue
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("include_cpp", expand_include_cpp);
}
