mod ffi;

use ffi::luau;

fn main() {
    println!("Hello, world!");
    luau::foo();
}
