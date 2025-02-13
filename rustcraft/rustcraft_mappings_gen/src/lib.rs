use jni::objects::JObject;
use jni::sys::jint;
use jni::JNIEnv;
use jni::JNIVersion;

#[no_mangle]
extern "system" fn JNI_OnLoad(mut _env: JNIEnv) -> jint {
    JNIVersion::V8.into()
}

#[no_mangle]
extern "system" fn Java_fr_supersurviveur_mappingsmod_Mappingsmod_run(
    mut env: JNIEnv,
    _obj: JObject,
) {
    println!("Starting mappings completion...");

    let mut mappings = rustcraft_mappings::parse_mappings();

    let java_utils = env
        .find_class("fr/supersurviveur/mappingsmod/ASMUtils")
        .unwrap();

    for c in mappings.mapped_map.values_mut() {
        for method in c.methods.values_mut() {
            let modifiers = env
                .call_static_method(
                    &java_utils,
                    "getModifiers",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)C",
                    &[
                        (&env.new_string(&c.mapped_name).unwrap()).into(),
                        (&env.new_string(&method.mapped_name).unwrap()).into(),
                        (&env.new_string(&method.mapped_signature).unwrap()).into(),
                    ],
                )
                .inspect_err(|_| env.exception_describe().unwrap())
                .unwrap()
                .c()
                .unwrap();
            for (i, arg) in method.args.iter_mut().enumerate() {
                let modifiers = env
                    .call_static_method(
                        &java_utils,
                        "getModifiers",
                        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;I)C",
                        &[
                            (&env.new_string(&c.mapped_name).unwrap()).into(),
                            (&env.new_string(&method.mapped_name).unwrap()).into(),
                            (&env.new_string(&method.mapped_signature).unwrap()).into(),
                            (i as jint).into(),
                        ],
                    )
                    .inspect_err(|_| env.exception_describe().unwrap())
                    .unwrap()
                    .c()
                    .unwrap();
                arg.modifiers = modifiers as u8;
            }
            method.modifiers = modifiers as u8;
        }
        for field in c.fields.values_mut() {
            let modifiers = env
                .call_static_method(
                    &java_utils,
                    "getModifiers",
                    "(Ljava/lang/String;Ljava/lang/String;)C",
                    &[
                        (&env.new_string(&c.mapped_name).unwrap()).into(),
                        (&env.new_string(&field.mapped_name).unwrap()).into(),
                    ],
                )
                .inspect_err(|_| env.exception_describe().unwrap())
                .unwrap()
                .c()
                .unwrap();
            field.modifiers = modifiers as u8;
        }
    }

    rustcraft_mappings::set_mappings(&mappings);

    println!("Mappings completion complete !");
}
