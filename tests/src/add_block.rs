use rustcraft::api::block::{prelude::*, ActionResult};
use rustcraft_codegen::block;

#[block]
pub struct MyBlock {
    pub name: String,
}

impl Block for MyBlock {
    fn on_use(&self, api: &mut ModApi) -> ActionResult {
        println!("From my block, when this is displayed, it means that it works ! reload ?");
        return ActionResult::CONSUME;
    }
}
