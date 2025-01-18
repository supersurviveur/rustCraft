pub mod block;
pub mod object;

use block::Block;
use jni::objects::{JObject, JValue, JValueGen};
use jni::sys::jlong;
use jni::JNIEnv;

pub struct ModApi<'local> {
    pub(crate) current_env: JNIEnv<'local>,
    pub(crate) current_obj: JObject<'local>,
}

impl<'local> ModApi<'local> {
    pub(crate) fn new(env: JNIEnv<'local>, obj: JObject<'local>) -> ModApi<'local> {
        ModApi {
            current_env: env,
            current_obj: obj,
        }
    }
}
impl<'local> ModApi<'local> {
    fn call_method(
        &mut self,
        object: Option<&JObject>,
        name: &str,
        signature: &str,
        args: &[JValue],
    ) -> JValueGen<JObject<'local>> {
        // TODO manage exceptions
        self.current_env
            .call_method(object.unwrap_or(&self.current_obj), name, signature, args)
            .unwrap()
    }
    fn get_block_class(&mut self) -> JObject<'local> {
        let instance = self
            .call_method(
                None,
                "getBlockAPI",
                "()Lfr/supersurviveur/rustcraftmod/rustapi/rustblock/BlockAPI;",
                &[],
            )
            .l()
            .unwrap();
        return instance;
    }

    pub fn register_block<T: Block>(&mut self, block: T) {
        let pointer: Box<dyn Block> = Box::new(block);
        let pointer = Box::new(pointer);
        let pointer = Box::into_raw(pointer);
        let c = self.get_block_class();
        self.current_env
            .call_method(&c, "createBlock", "(J)V", &[(pointer as jlong).into()])
            .unwrap();
    }
}
