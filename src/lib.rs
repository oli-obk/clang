extern crate libc;

use libc::c_int;

#[link(name = "clang")]
extern {
    pub fn clang_createIndex(excludeDeclarationsFromPCH: c_int, displayDiagnostics: c_int);
}
