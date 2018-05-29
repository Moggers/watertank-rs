use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::fs;
extern crate bindgen;
extern crate cc;
extern crate fs_extra;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let inc = &PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("inc");
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!(
        "cargo:rustc-link-search={}",
        inc.join("component/soc/realtek/8195a/misc/bsp/lib/common/GCC")
            .display()
    );
    println!("cargo:rustc-link-lib=static=p2p");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
