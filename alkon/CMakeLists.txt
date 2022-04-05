cmake_minimum_required(VERSION 3.0)
project(FFI LANGUAGES CXX)

set(RUST_BINDINGS_DIR target/cxxbridge/alkon/src)
set(LUAU_DIR ../luau)
                
add_library(FFI STATIC ${RUST_BINDINGS_DIR}/ffi.rs.cc ${RUST_BINDINGS_DIR}/ffi.rs.h)
target_include_directories(FFI PUBLIC 
                           ${LUAU_DIR}/Analysis/include/Luau 
                           ${LUAU_DIR}/Ast/include/Luau
                           ${LUAU_DIR}/CLI)
target_compile_features(FFI PUBLIC cxx_std_17)

set_property(TARGET FFI PROPERTY
    MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")

install(TARGETS FFI DESTINATION lib)