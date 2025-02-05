use std::ffi::c_void;

use crate::world::World;
mod action_result;
pub use action_result::*;

pub mod prelude;

use crate::api::ModApi;

pub trait Block {
    fn register(&self) -> Vec<(&str, &str, *mut c_void)> {
        panic!("Macro should be called on blocks !");
    }
    fn on_stepped_on(&mut self, _api: ModApi, _world: World) {}
    fn on_use(&mut self, _api: ModApi, _block_state: World, _world: World) -> ActionResult {
        return ActionResult::Pass;
    }
}

pub trait BlockEntity {
    fn register(&self) -> Vec<(&str, &str, *mut c_void)> {
        panic!("Macro should be called on blocks !");
    }
    fn on_stepped_on(&mut self, _api: ModApi, _world: World) {}
    fn on_use(&mut self, _api: ModApi, _block_state: World, _world: World) -> ActionResult {
        return ActionResult::Pass;
    }
}
