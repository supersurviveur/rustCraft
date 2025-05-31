use rustcraft::{
    block::{prelude::*, ActionResult},
    net::minecraft::client::color::world,
    prelude::JObject,
};
pub struct MyBlock {
    pub i: u64,
}
#[block]
impl Block for MyBlock {
    fn on_stepped_on<'a>(&mut self, world: &'a mut rustcraft::net::minecraft::world::World<'a>) {
        self.i += 1;
        println!("{}", world.is_night());
        if !world.is_client() {
            // world.get_server().unwrap().get_player_manager().broadcast(
            //     format!(
            //         "When this is displayed, it means that it works ! cpt: {} + is_night : {} + sky: {}",
            //         self.i,
            //         1, 1
            //         // test.is_night(),
            //         // test.is_client()
            //     )
            //     .as_str(),
            //     false,
            // )
        }
        // let test = rustcraft::net::minecraft::world::WorldAccess::new(world.api.clone(), unsafe {
        //     JObject::from_raw(world.inner.clone())
        // });
        // test.get_server();
    }
    fn on_use<'a>(
        &mut self,
        _block_state: &'a mut rustcraft::net::minecraft::world::World<'a>,
        _world: &'a mut rustcraft::net::minecraft::world::World<'a>,
    ) -> ActionResult {
        // if !world.is_client() {
        //     world
        //         .get_server()
        //         .unwrap()
        //         .get_player_manager()
        //         .broadcast("hello world !", false)
        // }
        return ActionResult::Consume;
    }
}
