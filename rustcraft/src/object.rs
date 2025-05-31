use jni::objects::JObject;

use super::ModApi;

pub trait RustObject {}

/// Retrieves a rust object from a leaked box in a java class
pub fn load_object<'a, T: ?Sized>(api: ModApi, java_class: Option<&JObject>) -> &'a mut Box<T> {
    let r = api.get_field(java_class, ("rust_object", "J"));
    let object = r.j().unwrap();
    let object = object as *mut Box<T>;
    let object = unsafe { &mut *object };
    return object;
}

pub trait ToJava {
    fn to_java<'local>(self, api: ModApi<'local>) -> JObject<'local>;
}

pub trait JavaObject<'a> {
    fn get_inner(&self) -> &JObject<'_>;
    fn get_api(&self) -> ModApi<'a>;
}
