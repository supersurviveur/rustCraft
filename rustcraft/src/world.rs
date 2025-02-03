use jni::objects::JObject;
use rustcraft_codegen::mappings_with_sig;

use crate::{api::ModApi, minecraft_server::MinecraftServer};

#[derive(Debug)]
pub struct World<'a> {
    api: ModApi<'a>,
    inner: JObject<'a>,
}

impl<'a> World<'a> {
    pub fn new(api: ModApi<'a>, world: JObject<'a>) -> Self {
        World { api, inner: world }
    }
    pub fn get_server(&self) -> Option<MinecraftServer<'a>> {
        let value = self
            .api
            .call_method(
                Some(&self.inner),
                mappings_with_sig!("net/minecraft/world/WorldAccess", "getServer"),
                &[],
            )
            .l()
            .unwrap();
        if value.is_null() {
            None
        } else {
            Some(MinecraftServer::new(self.api.clone(), value))
        }
    }
    pub fn is_client(&self) -> bool {
        self.api
            .get_field(Some(&self.inner), ("isClient", "Z"))
            .z()
            .unwrap()
    }
}
