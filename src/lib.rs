#![allow(non_snake_case)]

extern crate libc;

use libc::{c_void, c_int, c_char, c_uint};

pub type CXIndex = *mut c_void;
pub type CXCursorVisitor = extern fn(cursor: CXCursor, parent: CXCursor, client_data: CXClientData) -> CXChildVisitResult;
pub type CXClientData = *mut c_void;

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
    pub fn clang_parseTranslationUnit2(
        CIdx: CXIndex,
        source_filename: *const c_char,
        command_line_args: *const *const c_char,
        num_command_line_args: c_int,
        unsaved_files: *const CXUnsavedFile,
        num_unsaved_files: c_uint,
        options: c_uint,
        out_TU: *mut CXTranslationUnit,
    ) -> CXErrorCode;
    pub fn clang_getTranslationUnitCursor(
        tu: CXTranslationUnit,
    ) -> CXCursor;
    pub fn clang_visitChildren(
        parent: CXCursor,
        visitor: CXCursorVisitor,
        client_data: CXClientData,
    ) -> c_uint;
    pub fn clang_getCursorType(
        cursor: CXCursor
    ) -> CXType;
    pub fn clang_getCString(
        string: CXString,
    ) -> *const c_char;
    pub fn clang_Cursor_getMangling(
        cursor: CXCursor,
    ) -> CXString;
    pub fn clang_getCursorSpelling(
        cursor: CXCursor
    ) -> CXString;
    pub fn clang_getResultType(
        T: CXType
    ) -> CXType;
    pub fn clang_getTypedefDeclUnderlyingType(
        C: CXCursor,
    ) -> CXType;
    pub fn clang_getTypeDeclaration(
        T: CXType,
    ) -> CXCursor;
    pub fn clang_getTypeSpelling(
        T: CXType
    ) -> CXString;
    pub fn clang_getPointeeType(
        T: CXType
    ) -> CXType;
    pub fn clang_isConstQualifiedType(
        T: CXType
    ) -> c_uint;
}

#[repr(C)]
pub struct CXString {
    data: *const c_void,
    private_flags: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum CXTypeKind {
    CXType_Invalid = 0,
    CXType_Unexposed = 1,
    CXType_Void = 2,
    CXType_Bool = 3,
    CXType_Char_U = 4,
    CXType_UChar = 5,
    CXType_Char16 = 6,
    CXType_Char32 = 7,
    CXType_UShort = 8,
    CXType_UInt = 9,
    CXType_ULong = 10,
    CXType_ULongLong = 11,
    CXType_UInt128 = 12,
    CXType_Char_S = 13,
    CXType_SChar = 14,
    CXType_WChar = 15,
    CXType_Short = 16,
    CXType_Int = 17,
    CXType_Long = 18,
    CXType_LongLong = 19,
    CXType_Int128 = 20,
    CXType_Float = 21,
    CXType_Double = 22,
    CXType_LongDouble = 23,
    CXType_NullPtr = 24,
    CXType_Overload = 25,
    CXType_Dependent = 26,
    CXType_ObjCId = 27,
    CXType_ObjCClass = 28,
    CXType_ObjCSel = 29,
    CXType_Complex = 100,
    CXType_Pointer = 101,
    CXType_BlockPointer = 102,
    CXType_LValueReference = 103,
    CXType_RValueReference = 104,
    CXType_Record = 105,
    CXType_Enum = 106,
    CXType_Typedef = 107,
    CXType_ObjCInterface = 108,
    CXType_ObjCObjectPointer = 109,
    CXType_FunctionNoProto = 110,
    CXType_FunctionProto = 111,
    CXType_ConstantArray = 112,
    CXType_Vector = 113,
    CXType_IncompleteArray = 114,
    CXType_VariableArray = 115,
    CXType_DependentSizedArray = 116,
    CXType_MemberPointer = 117,
}

#[repr(C)]
#[derive(Clone)]
pub struct CXType {
    pub kind: CXTypeKind,
    data: [*const c_void; 2],
}

#[repr(C)]
pub enum CXChildVisitResult {
    CXChildVisit_Break,
    CXChildVisit_Continue,
    CXChildVisit_Recurse,
}

#[repr(C)]
#[derive(Clone)]
pub struct CXCursor {
    pub kind: CXCursorKind,
    xdata: c_int,
    data: [*const c_void; 3],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum CXCursorKind {
    CXCursor_UnexposedDecl = 1,
    CXCursor_StructDecl = 2,
    CXCursor_UnionDecl = 3,
    CXCursor_ClassDecl = 4,
    CXCursor_EnumDecl = 5,
    CXCursor_FieldDecl = 6,
    CXCursor_EnumConstantDecl = 7,
    CXCursor_FunctionDecl = 8,
    CXCursor_VarDecl = 9,
    CXCursor_ParmDecl = 10,
    CXCursor_ObjCInterfaceDecl = 11,
    CXCursor_ObjCCategoryDecl = 12,
    CXCursor_ObjCProtocolDecl = 13,
    CXCursor_ObjCPropertyDecl = 14,
    CXCursor_ObjCIvarDecl = 15,
    CXCursor_ObjCInstanceMethodDecl = 16,
    CXCursor_ObjCClassMethodDecl = 17,
    CXCursor_ObjCImplementationDecl = 18,
    CXCursor_ObjCCategoryImplDecl = 19,
    CXCursor_TypedefDecl = 20,
    CXCursor_CXXMethod = 21,
    CXCursor_Namespace = 22,
    CXCursor_LinkageSpec = 23,
    CXCursor_Constructor = 24,
    CXCursor_Destructor = 25,
    CXCursor_ConversionFunction = 26,
    CXCursor_TemplateTypeParameter = 27,
    CXCursor_NonTypeTemplateParameter = 28,
    CXCursor_TemplateTemplateParameter = 29,
    CXCursor_FunctionTemplate = 30,
    CXCursor_ClassTemplate = 31,
    CXCursor_ClassTemplatePartialSpecialization = 32,
    CXCursor_NamespaceAlias = 33,
    CXCursor_UsingDirective = 34,
    CXCursor_UsingDeclaration = 35,
    CXCursor_TypeAliasDecl = 36,
    CXCursor_ObjCSynthesizeDecl = 37,
    CXCursor_ObjCDynamicDecl = 38,
    CXCursor_CXXAccessSpecifier = 39,
    CXCursor_ObjCSuperClassRef = 40,
    CXCursor_ObjCProtocolRef = 41,
    CXCursor_ObjCClassRef = 42,
    CXCursor_TypeRef = 43,
    CXCursor_CXXBaseSpecifier = 44,
    CXCursor_TemplateRef = 45,
    CXCursor_NamespaceRef = 46,
    CXCursor_MemberRef = 47,
    CXCursor_LabelRef = 48,
    CXCursor_OverloadedDeclRef = 49,
    CXCursor_VariableRef = 50,
    CXCursor_InvalidFile = 70,
    CXCursor_NoDeclFound = 71,
    CXCursor_NotImplemented = 72,
    CXCursor_InvalidCode = 73,
    CXCursor_UnexposedExpr = 100,
    CXCursor_DeclRefExpr = 101,
    CXCursor_MemberRefExpr = 102,
    CXCursor_CallExpr = 103,
    CXCursor_ObjCMessageExpr = 104,
    CXCursor_BlockExpr = 105,
    CXCursor_IntegerLiteral = 106,
    CXCursor_FloatingLiteral = 107,
    CXCursor_ImaginaryLiteral = 108,
    CXCursor_StringLiteral = 109,
    CXCursor_CharacterLiteral = 110,
    CXCursor_ParenExpr = 111,
    CXCursor_UnaryOperator = 112,
    CXCursor_ArraySubscriptExpr = 113,
    CXCursor_BinaryOperator = 114,
    CXCursor_CompoundAssignOperator = 115,
    CXCursor_ConditionalOperator = 116,
    CXCursor_CStyleCastExpr = 117,
    CXCursor_CompoundLiteralExpr = 118,
    CXCursor_InitListExpr = 119,
    CXCursor_AddrLabelExpr = 120,
    CXCursor_StmtExpr = 121,
    CXCursor_GenericSelectionExpr = 122,
    CXCursor_GNUNullExpr = 123,
    CXCursor_CXXStaticCastExpr = 124,
    CXCursor_CXXDynamicCastExpr = 125,
    CXCursor_CXXReinterpretCastExpr = 126,
    CXCursor_CXXConstCastExpr = 127,
    CXCursor_CXXFunctionalCastExpr = 128,
    CXCursor_CXXTypeidExpr = 129,
    CXCursor_CXXBoolLiteralExpr = 130,
    CXCursor_CXXNullPtrLiteralExpr = 131,
    CXCursor_CXXThisExpr = 132,
    CXCursor_CXXThrowExpr = 133,
    CXCursor_CXXNewExpr = 134,
    CXCursor_CXXDeleteExpr = 135,
    CXCursor_UnaryExpr = 136,
    CXCursor_ObjCStringLiteral = 137,
    CXCursor_ObjCEncodeExpr = 138,
    CXCursor_ObjCSelectorExpr = 139,
    CXCursor_ObjCProtocolExpr = 140,
    CXCursor_ObjCBridgedCastExpr = 141,
    CXCursor_PackExpansionExpr = 142,
    CXCursor_SizeOfPackExpr = 143,
    CXCursor_LambdaExpr = 144,
    CXCursor_ObjCBoolLiteralExpr = 145,
    CXCursor_ObjCSelfExpr = 146,
    CXCursor_UnexposedStmt = 200,
    CXCursor_LabelStmt = 201,
    CXCursor_CompoundStmt = 202,
    CXCursor_CaseStmt = 203,
    CXCursor_DefaultStmt = 204,
    CXCursor_IfStmt = 205,
    CXCursor_SwitchStmt = 206,
    CXCursor_WhileStmt = 207,
    CXCursor_DoStmt = 208,
    CXCursor_ForStmt = 209,
    CXCursor_GotoStmt = 210,
    CXCursor_IndirectGotoStmt = 211,
    CXCursor_ContinueStmt = 212,
    CXCursor_BreakStmt = 213,
    CXCursor_ReturnStmt = 214,
    CXCursor_GCCAsmStmt = 215,
    CXCursor_ObjCAtTryStmt = 216,
    CXCursor_ObjCAtCatchStmt = 217,
    CXCursor_ObjCAtFinallyStmt = 218,
    CXCursor_ObjCAtThrowStmt = 219,
    CXCursor_ObjCAtSynchronizedStmt = 220,
    CXCursor_ObjCAutoreleasePoolStmt = 221,
    CXCursor_ObjCForCollectionStmt = 222,
    CXCursor_CXXCatchStmt = 223,
    CXCursor_CXXTryStmt = 224,
    CXCursor_CXXForRangeStmt = 225,
    CXCursor_SEHTryStmt = 226,
    CXCursor_SEHExceptStmt = 227,
    CXCursor_SEHFinallyStmt = 228,
    CXCursor_MSAsmStmt = 229,
    CXCursor_NullStmt = 230,
    CXCursor_DeclStmt = 231,
    CXCursor_OMPParallelDirective = 232,
    CXCursor_OMPSimdDirective = 233,
    CXCursor_OMPForDirective = 234,
    CXCursor_OMPSectionsDirective = 235,
    CXCursor_OMPSectionDirective = 236,
    CXCursor_OMPSingleDirective = 237,
    CXCursor_OMPParallelForDirective = 238,
    CXCursor_OMPParallelSectionsDirective = 239,
    CXCursor_OMPTaskDirective = 240,
    CXCursor_OMPMasterDirective = 241,
    CXCursor_OMPCriticalDirective = 242,
    CXCursor_OMPTaskyieldDirective = 243,
    CXCursor_OMPBarrierDirective = 244,
    CXCursor_OMPTaskwaitDirective = 245,
    CXCursor_OMPFlushDirective = 246,
    CXCursor_SEHLeaveStmt = 247,
    CXCursor_OMPOrderedDirective = 248,
    CXCursor_OMPAtomicDirective = 249,
    CXCursor_OMPForSimdDirective = 250,
    CXCursor_OMPParallelForSimdDirective = 251,
    CXCursor_OMPTargetDirective = 252,
    CXCursor_OMPTeamsDirective = 253,
    CXCursor_TranslationUnit = 300,
    CXCursor_UnexposedAttr = 400,
    CXCursor_IBActionAttr = 401,
    CXCursor_IBOutletAttr = 402,
    CXCursor_IBOutletCollectionAttr = 403,
    CXCursor_CXXFinalAttr = 404,
    CXCursor_CXXOverrideAttr = 405,
    CXCursor_AnnotateAttr = 406,
    CXCursor_AsmLabelAttr = 407,
    CXCursor_PackedAttr = 408,
    CXCursor_PureAttr = 409,
    CXCursor_ConstAttr = 410,
    CXCursor_NoDuplicateAttr = 411,
    CXCursor_CUDAConstantAttr = 412,
    CXCursor_CUDADeviceAttr = 413,
    CXCursor_CUDAGlobalAttr = 414,
    CXCursor_CUDAHostAttr = 415,
    CXCursor_CUDASharedAttr = 416,
    CXCursor_PreprocessingDirective = 500,
    CXCursor_MacroDefinition = 501,
    CXCursor_MacroExpansion = 502,
    CXCursor_InclusionDirective = 503,
    CXCursor_ModuleImportDecl = 600,
    CXCursor_OverloadCandidate = 700,
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
#[derive(Debug)]
pub enum CXErrorCode {
    CXError_Success = 0,
    CXError_Failure = 1,
    CXError_Crashed = 2,
    CXError_InvalidArguments = 3,
    CXError_ASTReadError = 4
}
