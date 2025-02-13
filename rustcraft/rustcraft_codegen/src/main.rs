use rustcraft_mappings::codegen::auto_gen_impl;
use std::{fs, path::Path};
use syn::parse::Parse;

pub fn main() {
    let result = auto_gen_impl();
    // let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let current_dir = Path::new("/home/julien/code/rustCraft/rustcraft/src");
    println!("Writted in {:?}", current_dir);
    fs::write(
        current_dir.join("test.rs"),
        // prettyplease::unparse(&result),
        result.to_string(),
    )
    .unwrap();
}
