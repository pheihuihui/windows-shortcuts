use std::{env, fs, path::Path};

#[allow(unused_macros)]
macro_rules! build_print {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&src_dir);
    let profile = env::var("PROFILE").unwrap();

    let release_dir = src_dir.join("target").join(profile);
    if release_dir.exists() {
        let config = release_dir
            .join("config.txt")
            .as_path()
            .display()
            .to_string();
        let src_config = src_dir.join("config.txt").as_path().display().to_string();
        fs::copy(src_config, config).unwrap();
    }
}
