// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("as").arg("src/time.asm").arg("-o").arg(out_dir.clone()+"/rdtsc.o").status().unwrap();
    Command::new("ar").arg("crus").arg("librdtsc.a").arg( "rdtsc.o")
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=rdtsc");
//    println!("cargo:rustc-link-lib=dylib=atomic");
}
