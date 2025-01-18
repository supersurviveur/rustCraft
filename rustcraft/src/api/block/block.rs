use jni::{objects::JObject, sys::jlong, JNIEnv};

use crate::api::{
    object::{load_object, RustObject},
    ModApi,
};

use super::action_result::ActionResult;

pub trait BlockInternal {
    fn get_name(&self) -> String;
}

pub trait Block: RustObject + BlockInternal {
    fn on_use(&self, _api: &mut ModApi) -> ActionResult {
        return ActionResult::PASS;
    }
}

#[no_mangle]
pub extern "system" fn Java_fr_supersurviveur_rustcraftmod_rustapi_rustblock_RustBlock_onSteppedOn<
    'local,
>(
    env: JNIEnv<'local>,
    obj: JObject<'local>,
    block: JObject<'local>,
    block2: JObject<'local>,
    block3: JObject<'local>,
    block4: JObject<'local>,
) {
    let mut api = ModApi::new(env, block);
    let block: &Box<dyn Block> = load_object(&mut api, obj);

    let action_result = block.on_use(&mut api);
    println!("Block stepped on !");
}
