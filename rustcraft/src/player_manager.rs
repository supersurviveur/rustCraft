use jni::objects::JObject;
use rustcraft_codegen::mappings_with_sig;

use crate::api::ModApi;

#[derive(Debug)]
pub struct PlayerManager<'a> {
    api: ModApi<'a>,
    inner: JObject<'a>,
}

impl<'a> PlayerManager<'a> {
    pub fn new(api: ModApi<'a>, world: JObject<'a>) -> Self {
        PlayerManager { api, inner: world }
    }
    pub fn broadcast(&self, content: &str, overlay: bool) {
        let text = self.api.call_static_method(
            &self.api.get_class("net/minecraft/text/Text"),
            mappings_with_sig!(
                "net/minecraft/text/Text",
                "of#(Ljava/lang/String;)Lnet/minecraft/text/Text;"
            ),
            &[(&self.api.java_string(content)).into()],
        );
        self.api.call_method(
            Some(&self.inner),
            mappings_with_sig!(
                "net/minecraft/server/PlayerManager",
                "broadcast#(Lnet/minecraft/text/Text;Z)V"
            ),
            &[text.borrow(), overlay.into()],
        );
    }
}
