mod add_block;

use rustcraft::{api::ModApi, block::Block, prelude::*, register_block};

use crate::add_block::MyBlock;

#[rust_mod]
struct McMod;

impl rustcraft::RustCraftMod for McMod {
    fn on_enable(&self, api: &mut ModApi) {
        println!("Hello from Rust!");
        let block = MyBlock { i: 0 };
        register_block!(api, block, "dynamic", Block);
    }

    fn on_disable(&self, _api: &mut ModApi) {
        println!("Goodbye from Rust!");
    }

    fn new() -> Self {
        McMod {}
    }
}
