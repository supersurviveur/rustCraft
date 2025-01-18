use crate::JObject;
use crate::ModApi;
use rustcraft_codegen::to_java;

#[to_java("net/minecraft/class_1269")]
pub enum ActionResult {
    SUCCESS = 0,
    CONSUME = 1,
    CONSUME_PARTIAL = 2,
    PASS = 3,
    FAIL = 4,
}
