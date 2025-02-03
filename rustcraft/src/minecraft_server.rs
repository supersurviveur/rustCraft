use jni::objects::JObject;
use rustcraft_codegen::mappings_with_sig;

use crate::{api::ModApi, player_manager::PlayerManager};

#[derive(Debug)]
pub struct MinecraftServer<'a> {
    api: ModApi<'a>,
    inner: JObject<'a>,
}

impl<'a> MinecraftServer<'a> {
    pub(crate) fn new(api: ModApi<'a>, server: JObject<'a>) -> Self {
        MinecraftServer { api, inner: server }
    }
    pub fn get_player_manager(&self) -> PlayerManager<'a> {
        PlayerManager::new(
            self.api.clone(),
            self.api
                .call_method(
                    Some(&self.inner),
                    mappings_with_sig!("net/minecraft/server/MinecraftServer", "getPlayerManager"),
                    &[],
                )
                .l()
                .unwrap(),
        )
    }
}
