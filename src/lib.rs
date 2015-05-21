extern crate libc;

use libc::{c_void, c_int};

#[link(name = "clang")]
extern "C" {
    pub fn clang_createIndex(
        excludeDeclarationsFromPCH: c_int,
        displayDiagnostics: c_int,
    ) -> *mut c_void;
}
