use jni::objects::JObject;

use super::{block::Block, ModApi};

pub trait RustObject {}

/// Retrieves a rust object from a leaked box in a java class
pub(crate) fn load_object<'a, T: ?Sized>(api: &mut ModApi, java_class: JObject) -> &'a mut Box<T> {
    let r = api
        .current_env
        .get_field(java_class, "rust_object", "J")
        .unwrap();
    let object = r.j().unwrap();
    let object = object as *mut Box<T>;
    let object = unsafe { &mut *object };
    return object;
}
// pub(crate) fn load_object<'a, T>(api: &mut ModApi, java_class: JObject) -> &'a mut std::boxed::ThinBox<T> {
//     let r = api
//         .current_env
//         .get_field(java_class, "rust_object", "J")
//         .unwrap();
//     let object = r.j().unwrap();
//     let object = object as *mut <T>;
//     let object = unsafe { &mut *object };
//     return object;
// }
