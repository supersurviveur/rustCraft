use std::ffi::c_void;

mod action_result;
pub use action_result::*;

pub mod prelude;

pub trait Block {
    fn register(&self) -> Vec<(&str, &str, *mut c_void)> {
        panic!("Macro should be called on blocks !");
    }
    fn on_stepped_on<'a>(&mut self, _world: &'a mut crate::net::minecraft::world::World<'a>) {}
    fn on_use<'a>(
        &mut self,
        _block_state: &'a mut crate::net::minecraft::world::World<'a>,
        _world: &'a mut crate::net::minecraft::world::World<'a>,
    ) -> ActionResult {
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
