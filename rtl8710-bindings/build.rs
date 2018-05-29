use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
extern crate bindgen;
extern crate cc;

fn main() {
    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .clang_arg("-Iinc/component/os/freertos/freertos_v8.1.2/Source/include")
        .clang_arg("-Iinc")
        .clang_arg("-Iinc/component/os/freertos/freertos_v8.1.2/Source/portable/GCC/ARM_CM3")
        .clang_arg("-Iinc/component/common/drivers/wlan/realtek/include")
        .clang_arg("-Iinc/component/common/api/platform")
        .clang_arg("-Iinc/component/common/drivers/wlan/realtek/src/osdep")
        .clang_arg("-Iinc/component/common/api")
        .header("inc/wrapper.h")
        .whitelist_function("wifi_connect")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
