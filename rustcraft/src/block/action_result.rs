use crate::JObject;
use crate::ModApi;
use rustcraft_codegen::to_java;

#[to_java("net/minecraft/util/ActionResult")]
pub enum ActionResult {
    Success,
    Consume,
    Pass,
    Fail,
}
