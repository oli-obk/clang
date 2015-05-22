#![feature(plugin, libc)]
#![plugin(cpp_bind_gen)]

mod clang {
    include_cpp!{"clang-c/CXString.h"}
}
