use rustcraft::{
    block::{prelude::*, ActionResult},
    prelude::JObject,
    world::World,
};

pub struct MyBlock {
    pub i: u64,
}

#[block]
impl Block for MyBlock {
    fn on_stepped_on<'a>(&mut self, _api: ModApi, world: World) {
        self.i += 1;
        let test = rustcraft::net::minecraft::world::World::new(
            _api.clone(),
            _api.new_local_ref(unsafe { std::mem::transmute::<&JObject, &JObject>(&world.inner) }),
        );
        if !world.is_client() {
            world.get_server().unwrap().get_player_manager().broadcast(
                format!(
                    "When this is displayed, it means that it works ! cpt: {} + is_night : {} + is_client: {}",
                    self.i,
                    test.is_night(),
                    test.get_is_client()
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
