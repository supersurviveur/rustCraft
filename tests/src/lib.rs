mod add_block;

use rustcraft::{api::ModApi, prelude::*};

use crate::add_block::MyBlock;

#[rust_mod]
struct McMod;

impl rustcraft::RustCraftMod for McMod {
    fn on_enable(&self, api: &mut ModApi) {
        println!("Hello from Rust! with");
        let block = MyBlock {
            name: "test_rust".to_string(),
        };
        api.register_block(block);
    }

    fn on_disable(&self, _api: &mut ModApi) {
        println!("Goodbye from Rust!");
    }

    fn new() -> Self {
        McMod {}
    }
}
