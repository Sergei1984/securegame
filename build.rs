use std::env;
use std::fs::*;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dst = Path::new(&out_dir).join("../../../assets/");
    let src = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("assets/");

    let r = copy_recursive(&src, &dst);

    println!("cargo:warning=COPY RESULT is {:?}", r);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/fonts/FiraMono-Medium.ttf");
}

fn copy_recursive(from_dir: &PathBuf, to_dir: &PathBuf) -> std::io::Result<u64> {
    println!(
        "cargo:warning=COPY RECURSIVE {:?} to {:?}",
        &from_dir, &to_dir
    );

    if !to_dir.exists() {
        create_dir_all(&to_dir)?;
    }

    for item in read_dir(from_dir)? {
        let path = item?;
        let dest = to_dir.join(path.file_name());

        if path.file_type().unwrap().is_dir() {
            println!("cargo:warning=CREATE DIRECTORY {:?}", &dest);
            if !dest.exists() {
                create_dir_all(&dest)?;
            }
            copy_recursive(&path.path(), &dest)?;
        } else {
            println!("cargo:warning=COPY FILE {:?} to {:?}", path, &dest);
            copy(path.path(), dest)?;
        }
    }
    Ok(0)
}
