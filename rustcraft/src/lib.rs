use std::sync::Mutex;

use api::ModApi;
use jni::objects::JObject;
use jni::JNIEnv;

pub use rustcraft_codegen::register_block;

pub mod api;
pub mod block;
pub mod object;
pub mod prelude;

pub mod block_pos;
pub mod minecraft_server;
pub mod player_manager;
pub mod registry;
pub mod test;
pub use test::com;
pub use test::net;
pub mod world;

// Define the base traits for the mod
pub trait RustCraftMod {
    fn new() -> Self
    where
        Self: Sized;
    fn on_enable(&self, _api: &mut ModApi) {}
    fn on_disable(&self, _api: &mut ModApi) {}
}

// Global variable who points to the mod struct defined by user. Use Mutex to be memory safe
static RUSTCRAFTMOD: Mutex<Option<&'static (dyn RustCraftMod + Sync)>> = Mutex::new(None);

pub fn set_mod(rustcraftmod: &'static (dyn RustCraftMod + Sync)) {
    *RUSTCRAFTMOD.lock().unwrap() = Some(rustcraftmod);
}

pub fn get_mod() -> &'static dyn RustCraftMod {
    match *RUSTCRAFTMOD.lock().unwrap() {
        Some(rustcraftmod) => rustcraftmod,
        None => panic!("TODO RustCraftMod not set"),
    }
}

#[no_mangle]
pub extern "system" fn Java_fr_supersurviveur_rustcraftmod_rustapi_RustAPI_onInitialize<'local>(
    env: JNIEnv<'local>,
    obj: JObject<'local>,
) {
    let mut api = ModApi::new(env, obj);

    let mc_mod = get_mod();

    mc_mod.on_enable(&mut api);
}
