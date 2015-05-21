#![allow(non_snake_case)]

extern crate libc;

use libc::{c_void, c_int, c_char, c_uint};

pub type CXIndex = *mut c_void;

#[link(name = "clang")]
extern "C" {
    pub fn clang_createIndex(
        excludeDeclarationsFromPCH: c_int,
        displayDiagnostics: c_int,
    ) -> CXIndex;
    pub fn clang_parseTranslationUnit(
        CIdx: CXIndex,
        source_filename: *const c_char,
        command_line_args: *const *const c_char,
        num_command_line_args: c_int,
        unsaved_files: *const CXUnsavedFile,
        num_unsaved_files: c_uint,
        options: c_uint,
    ) -> CXTranslationUnit;
}

#[repr(C)]
pub struct CXTranslationUnitImpl;

pub type CXTranslationUnit = *mut CXTranslationUnitImpl;

#[repr(C)]
pub struct CXUnsavedFile {
    Filename: *const c_char,
    Contents: *const c_char,
    Length: libc::c_ulong,
}

#[repr(C)]
pub enum CXErrorCode {
    CXError_Success = 0,
    CXError_Failure = 1,
    CXError_Crashed = 2,
    CXError_InvalidArguments = 3,
    CXError_ASTReadError = 4
}
