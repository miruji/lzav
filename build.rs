#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::process::{Command, ExitStatus};
use std::env;
use std::path::PathBuf;

fn main()
{
  let outPath: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());
  let currentPath: String = env::var("CARGO_MANIFEST_DIR").unwrap();

  let c: PathBuf = PathBuf::from(currentPath).join("src/lzav.c");
  let ll: PathBuf = outPath.join("lzav.ll");
  let bc: PathBuf = outPath.join("lzav.bc");
  let o: PathBuf = outPath.join("lzav.o");

  // Define the correct target for generating the .ll intermediate code
  // todo You should specify the system here so that llvm correctly compiles the code
  //      and it correctly links with this library
  let targetOs: String = env::var("CARGO_CFG_TARGET_OS").unwrap();
  let target: &str = match targetOs.as_str() {
    "windows" => "x86_64-w64-mingw32",
    "linux" => "x86_64-linux-gnu",
    _ => panic!("Unsupported target OS: {:?}", targetOs),
  };

  // 1. .c -> .ll
  let mut status: ExitStatus = Command::new("clang")
    .args([
      "-S",
      &format!("--target={}", target),
      "-emit-llvm",
      "-fPIC",
      c.to_str().unwrap(),
      "-o"
    ])
    .arg(&ll)
    .status()
    .expect("Failed to run clang");
  assert!(status.success(), "Compilation to .ll failed");

  // 2. .ll -> .bc
  status = Command::new("llvm-as")
    .arg(&ll)
    .arg("-o")
    .arg(&bc)
    .status()
    .expect("Failed to run llvm-as");
  assert!(status.success(), "Compilation to .bc failed");

  // 3. .bc -> .o (with -relocation-model=pic)
  status = Command::new("llc")
    .args(["-filetype=obj", "-relocation-model=pic"])
    .arg(&bc)
    .arg("-o")
    .arg(&o)
    .status()
    .expect("Failed to run llc");
  assert!(status.success(), "Compilation to .o failed");

  // 4. .o -> .a
  status = Command::new("ar")
    .args(["crus", "liblzav.a", "lzav.o"])
    .current_dir(&outPath)
    .status()
    .expect("Failed to run ar");
  assert!(status.success(), "Failed to create liblzav.a");

  // 5. Link configuration
  println!("cargo:rustc-link-search=native={}", outPath.display());
  println!("cargo:rustc-link-lib=static=lzav");
}

