use proc_macro::TokenStream;
use syn::parse_macro_input;

mod to_java;

#[proc_macro_attribute]
pub fn rust_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let user_mod = parse_macro_input!(item as syn::ItemStruct);
    let mod_name = user_mod.ident.clone();

    quote::quote! {
        #user_mod

        static _MOD: std::sync::LazyLock<#mod_name> = std::sync::LazyLock::new(|| #mod_name::new());
        #[no_mangle]
        extern "system" fn JNI_OnLoad(mut _env: JNIEnv) -> jint {
            rustcraft::set_mod(&*_MOD);

            JNIVersion::V8.into()
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn to_java(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Match input type to know which macro to call
    let item = parse_macro_input!(input as syn::Item);
    match item {
        syn::Item::Enum(item) => to_java::enum_to_java_impl(attr, item),
        _ => panic!("Type not supported"),
    }
}

#[proc_macro_attribute]
pub fn block(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Match input type to know which macro to call
    let item = parse_macro_input!(input as syn::Item);
    let mut output = quote::quote! {
        #item
    };
    match &item {
        syn::Item::Struct(item) => {
            let item_name = &item.ident;
            output.extend(quote::quote! {
                impl BlockInternal for #item_name {
                    fn get_name(&self) -> String {
                        self.name.clone()
                    }
                }
                impl RustObject for #item_name {}
            });
        }
        _ => panic!("Error?"),
    }
    output.into()
}
