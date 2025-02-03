pub fn main() {
    println!("cargo::rerun-if-env-changed=DEV_MAPPINGS");
}
