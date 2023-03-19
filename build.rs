extern crate embed_resource;
use std::env;

macro_rules! build_print {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    embed_resource::compile("hdpi_plotting.rc");

    let out_dir = env::var("OUT_DIR").unwrap();
    build_print!("{out_dir}");
}
