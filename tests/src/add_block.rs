use rustcraft::{
    block::{prelude::*, ActionResult},
    world::World,
};

pub struct MyBlock {
    pub i: u64,
}

#[block]
impl Block for MyBlock {
    fn on_stepped_on(&mut self, _api: ModApi, world: World) {
        self.i += 1;
        if !world.is_client() {
            world.get_server().unwrap().get_player_manager().broadcast(
                format!(
                    "When this is displayed, it means that it works ! cpt: {}",
                    self.i
                )
                .as_str(),
                false,
            )
        }
    }
    fn on_use(&mut self, _api: ModApi, _block_state: World, world: World) -> ActionResult {
        if !world.is_client() {
            world
                .get_server()
                .unwrap()
                .get_player_manager()
                .broadcast("hello world !", false)
        }
        return ActionResult::Consume;
    }
}
