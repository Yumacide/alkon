#[cxx::bridge]
pub mod luau {
    unsafe extern "C++" {
        include!("FileUtils.h");
        fn foo() -> bool;
    }
}
