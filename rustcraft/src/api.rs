use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::block::Block;
use jni::descriptors::Desc;
use jni::objects::{JClass, JObject, JString, JValue, JValueGen};
use jni::signature::TypeSignature;
use jni::sys::{jlong, jvalue};
use jni::{JNIEnv, NativeMethod};

#[derive(Debug)]
pub struct BaseApi<'local> {
    pub current_env: JNIEnv<'local>,
    pub current_obj: JObject<'local>,
}

#[derive(Clone, Debug)]
pub struct ModApi<'local> {
    pub api: Rc<RefCell<BaseApi<'local>>>,
}

impl<'local> ModApi<'local> {
    pub fn new(env: JNIEnv<'local>, obj: JObject<'local>) -> ModApi<'local> {
        ModApi {
            api: Rc::new(RefCell::new(BaseApi {
                current_env: env,
                current_obj: obj,
            })),
        }
    }
}
impl<'local> BaseApi<'local> {
    pub(crate) fn get_field<'a>(
        &mut self,
        java_class: Option<&'a JObject<'a>>,
        signature: (&str, &str),
    ) -> JValueGen<JObject<'a>>
    where
        'local: 'a,
    {
        self.current_env
            .get_field(
                java_class.unwrap_or(&self.current_obj),
                signature.0,
                signature.1,
            )
            .inspect_err(|_| self.current_env.exception_describe().unwrap())
            .expect(&format!(
                "Couldn't get field {}: {}",
                signature.0, signature.1
            ))
    }
    pub(crate) fn get_field_class<'a>(
        &mut self,
        class: Option<&str>,
        object: Option<&'a JObject<'a>>,
        signature: (&str, &str),
    ) -> JValueGen<JObject<'a>>
    where
        'local: 'a,
    {
        let class = class
            .map(|c| Desc::<JClass>::lookup(c, &mut self.current_env).unwrap())
            .unwrap_or(
                self.current_env.auto_local(
                    self.current_env
                        .get_object_class(object.unwrap_or(&self.current_obj))
                        .unwrap(),
                ),
            );
        let ret = TypeSignature::from_str(signature.1).unwrap().ret;

        self.current_env
            .get_field_unchecked(
                object.unwrap_or(&self.current_obj),
                (class, signature.0, signature.1),
                ret,
            )
            .inspect_err(|_| self.current_env.exception_describe().unwrap())
            .unwrap()
    }
    pub(crate) fn call_method<'a>(
        &mut self,
        object: Option<&'a JObject<'a>>,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JValueGen<JObject<'a>>
    where
        'local: 'a,
    {
        self.current_env
            .call_method(
                object.unwrap_or(&self.current_obj),
                signature.0,
                signature.1,
                args,
            )
            .inspect_err(|_| self.current_env.exception_describe().unwrap())
            .unwrap()
    }
    pub(crate) fn call_method_class<'a>(
        &mut self,
        class: Option<&str>,
        object: Option<&'a JObject<'a>>,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JValueGen<JObject<'a>>
    where
        'local: 'a,
    {
        let class = class
            .map(|c| Desc::<JClass>::lookup(c, &mut self.current_env).unwrap())
            .unwrap_or(
                self.current_env.auto_local(
                    self.current_env
                        .get_object_class(object.unwrap_or(&self.current_obj))
                        .unwrap(),
                ),
            );
        let ret = TypeSignature::from_str(signature.1).unwrap().ret;
        let args: Vec<jvalue> = args.iter().map(|v| v.as_jni()).collect();
        unsafe {
            self.current_env.call_method_unchecked(
                object.unwrap_or(&self.current_obj),
                (class, signature.0, signature.1),
                ret,
                &args,
            )
        }
        .inspect_err(|_| self.current_env.exception_describe().unwrap())
        .unwrap()
    }
    pub(crate) fn call_method_object<'a>(
        &mut self,
        object: Option<&'a JObject<'a>>,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JObject<'a>
    where
        'local: 'a,
    {
        self.call_method(object, signature, args).l().unwrap()
    }

    pub(crate) fn get_block_manager(&mut self) -> JObject<'local> {
        let instance = self.call_method_object(
            None,
            (
                "getBlockAPI",
                "()Lfr/supersurviveur/rustcraftmod/rustapi/rustblock/BlockAPI;",
            ),
            &[],
        );
        return instance;
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
impl<'a> ModApi<'a> {
    pub(crate) fn get_field<'local>(
        &self,
        java_class: Option<&'local JObject<'local>>,
        signature: (&str, &str),
    ) -> JValueGen<JObject<'local>>
    where
        'a: 'local,
    {
        self.api.borrow_mut().get_field(java_class, signature)
    }
    pub(crate) fn get_field_class<'local>(
        &self,
        class: Option<&str>,
        object: Option<&'local JObject<'local>>,
        signature: (&str, &str),
    ) -> JValueGen<JObject<'local>>
    where
        'a: 'local,
    {
        self.api
            .borrow_mut()
            .get_field_class(class, object, signature)
    }
    pub(crate) fn call_method<'local>(
        &self,
        object: Option<&'local JObject<'local>>,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JValueGen<JObject<'local>>
    where
        'a: 'local,
    {
        (*self.api)
            .borrow_mut()
            .call_method(object, signature, args)
    }
    pub(crate) fn call_method_class<'local>(
        &self,
        class: Option<&str>,
        object: Option<&'local JObject<'local>>,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JValueGen<JObject<'local>>
    where
        'a: 'local,
    {
        self.api
            .borrow_mut()
            .call_method_class(class, object, signature, args)
    }
    pub(crate) fn get_block_manager(&self) -> JObject<'a> {
        (*self.api).borrow_mut().get_block_manager()
    }
    pub(crate) fn get_api_manager(&self) -> JClass {
        self.get_class("fr/supersurviveur/rustcraftmod/rustapi/RustAPI")
    }
    pub(crate) fn java_string(&self, s: &str) -> JString<'a> {
        (*self.api)
            .borrow_mut()
            .current_env
            .new_string(s)
            .expect("Error while creating a java string")
    }

    pub fn get_class(&self, class: &str) -> JClass {
        (*self.api)
            .borrow_mut()
            .current_env
            .find_class(class)
            .expect(&format!("Couldn't find class {}", class))
    }

    pub fn get_static_field(
        &self,
        class: &JClass,
        signature: (&str, &str),
    ) -> JValueGen<JObject<'a>> {
        (*self.api)
            .borrow_mut()
            .current_env
            .get_static_field(class, signature.0, signature.1)
            .expect(&format!(
                "Couldn't get field {}: {}",
                signature.0, signature.1
            ))
    }
    pub(crate) fn call_static_method(
        &self,
        class: &JClass,
        signature: (&str, &str),
        args: &[JValue],
    ) -> JValueGen<JObject<'a>> {
        (*self.api)
            .borrow_mut()
            .current_env
            .call_static_method(class, signature.0, signature.1, args)
            .unwrap()
    }

    /// Create a java class and register native methods on it
    pub fn register_block_natives(
        &self,
        register: Vec<(&str, &str, *mut c_void)>,
        super_class: &str,
    ) -> JObject<'a> {
        let methods_count = register.len();
        let methods = register
            .iter()
            .map(|(name, sig, fn_ptr)| NativeMethod {
                name: name.into(),
                sig: sig.into(),
                fn_ptr: fn_ptr.clone(),
            })
            .collect::<Vec<NativeMethod>>();

        let mut api = (*self.api).borrow_mut();
        let methods_names = api
            .current_env
            .new_object_array(
                methods_count.try_into().unwrap(),
                "Ljava/lang/String;",
                JObject::null(),
            )
            .inspect_err(|_| api.current_env.exception_describe().unwrap())
            .unwrap();

        let methods_sig = api
            .current_env
            .new_object_array(
                methods_count.try_into().unwrap(),
                "Ljava/lang/String;",
                JObject::null(),
            )
            .unwrap();

        register.iter().enumerate().for_each(|(i, (name, sig, _))| {
            api.current_env
                .set_object_array_element(
                    &methods_names,
                    i.try_into().unwrap(),
                    api.current_env.new_string(name).unwrap(),
                )
                .unwrap();

            api.current_env
                .set_object_array_element(
                    &methods_sig,
                    i.try_into().unwrap(),
                    api.current_env.new_string(sig).unwrap(),
                )
                .unwrap();
        });
        drop(api);

        let super_class = self.java_string(super_class);
        let class_name = self.java_string(format!("DynamicClass{}", get_id()).as_str());
        let new_class = self
            .call_method(
                None,
                (
                    "makeClass",
                    "(Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Class;",
                ),
                &[
                    (&class_name).into(),
                    (&methods_names).into(),
                    (&methods_sig).into(),
                    (&super_class).into(),
                ],
            )
            .l()
            .unwrap();
        let new_class_ref = (*self.api)
            .borrow_mut()
            .current_env
            .new_local_ref(&new_class)
            .unwrap();
        let mut env = (*self.api).borrow_mut();

        env.current_env
            .register_native_methods(JClass::from(new_class), methods.as_slice())
            .inspect_err(|_| env.current_env.exception_describe().unwrap())
            .unwrap();
        drop(env);
        new_class_ref
    }

    pub fn register_block<T: Block>(&self, new_class_ref: JObject, class_name: &str, block: T) {
        let pointer: Box<dyn Block> = Box::new(block);
        let pointer = Box::new(pointer);
        let pointer = Box::into_raw(pointer);

        let class_name = self.java_string(class_name);
        let c = self.get_block_manager();
        self.call_method(
            Some(&c),
            ("createBlock", "(JLjava/lang/String;Ljava/lang/Class;)V"),
            &[
                (pointer as jlong).into(),
                (&class_name).into(),
                (&new_class_ref).into(),
            ],
        );
    }

    pub fn info(&self, s: &str) {
        self.call_static_method(
            &self.get_api_manager(),
            ("info", "(Ljava/lang/String;)V"),
            &[(&self.java_string(s)).into()],
        );
    }
    pub fn new_local_ref<O: AsRef<JObject<'a>>>(&self, value: &O) -> JObject<'a> {
        (*self.api)
            .borrow_mut()
            .current_env
            .new_local_ref(value)
            .unwrap()
    }
}
