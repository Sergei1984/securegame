use std::env;
use std::fs::*;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../assets/fonts");

    let source_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("assets/fonts/FiraMono-Medium.ttf");

    println!("cargo:warning=OUT_PATH is {:?}", &dest_path);
    println!("cargo:warning=OUT_PATH is {:?}", &source_path);

    if !dest_path.is_dir() {
        let _ = create_dir_all(&dest_path);
    }

    let r = copy(source_path, dest_path.join("FiraMono-Medium.ttf"));

    println!("cargo:warning=COPY RESULT is {:?}", r);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/fonts/FiraMono-Medium.ttf");
}
