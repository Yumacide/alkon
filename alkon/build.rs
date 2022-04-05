use std::{
    env,
    path::Path,
    process::{Command, Output},
};

fn run(command: &mut Command) -> Output {
    let output = command.output().expect("Could not run cmake");

    if !output.status.success() {
        panic!("{:?}", output);
    }

    output
}

fn main() {let _build = cxx_build::bridge("src/ffi.rs");

    let build_type;
    let out_dir;
    match env::var("PROFILE").unwrap().as_str() {
        "debug" => {
            build_type = "RelWithDebInfo";
            out_dir = "target/debug";
        }

        "release" => {
            build_type = "Release";
            out_dir = "target/release";
        }

        _ => panic!("Unknown profile"),
    }

    if !Path::new(&(String::from(out_dir) + "CMakeCache.txt")).exists() {
        run(Command::new("cmake")
            .current_dir(out_dir)
            .arg("../../..") // root directory
            .arg(String::from("-DCMAKE_BUILD_TYPE=") + build_type));
    }

    let _output = run(Command::new("cmake")
        .current_dir(out_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg(build_type));

    println!("cargo:rustc-link-search=native={out_dir}/alkon/{build_type}");
    println!("cargo:rustc-link-search=native={out_dir}/luau/{build_type}");

    println!("cargo:rustc-link-lib=luau-analyze");
    println!("cargo:rustc-link-lib=luau-ast");
    println!("cargo:rustc-link-lib=Luau.Analysis");
    println!("cargo:rustc-link-lib=Luau.Ast");
    println!("cargo:rustc-link-lib=FFI");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=../luau");
}
