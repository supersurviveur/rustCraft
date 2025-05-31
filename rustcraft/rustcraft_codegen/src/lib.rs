use core::panic;
use std::{fs, path::Path};

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote};
use rustcraft_mappings::{
    codegen::auto_gen_impl, convert_sig, get_class, get_multiple_class, rust_to_java_method,
};
use syn::{
    parse::Parse, parse_macro_input, spanned::Spanned, ImplItem, ImplItemMethod, ItemImpl, LitStr,
    Token,
};

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
pub fn to_java_method(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as syn::ItemFn);
    let class = parse_macro_input!(attr as LitStr);

    let name = f.sig.ident;
    let name = name.to_string();
    let (name, method_name) = name.rsplit_once("_").unwrap();

    let binding = class.value();
    let mappings = get_class(&binding);

    f.sig.ident = format_ident!(
        "{}_{}",
        name,
        mappings
            .get_method(method_name)
            .expect(
                format!(
                    "Can't find method {} with and without signature",
                    method_name
                )
                .as_str()
            )
            .get_java_name()
            .replace("_", "_1")
    );
    quote::quote! {
        #f
    }
    .into()
}

struct MappingsParam {
    class: LitStr,
    name: LitStr,
}
impl Parse for MappingsParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let class = input.parse()?;
        input.parse::<Token![,]>()?;
        Ok(Self {
            class,
            name: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn mappings(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as MappingsParam);

    let binding = f.class.value();
    let mappings = get_class(&binding);

    let result = mappings
        .get_method(&f.name.value())
        .expect(
            format!(
                "Can't find method {} with and without signature",
                f.name.value()
            )
            .as_str(),
        )
        .get_java_name();
    quote::quote! {
        #result
    }
    .into()
}

#[proc_macro]
pub fn mappings_with_sig(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as MappingsParam);

    let binding = f.class.value();
    let mappings = get_class(&binding);

    let method = mappings.get_method(&f.name.value()).expect(
        format!(
            "Can't find method {} with and without signature",
            f.name.value()
        )
        .as_str(),
    );
    let method_name = method.get_java_name();
    let sig = method.get_java_sig();
    quote::quote! {
        (#method_name, #sig)
    }
    .into()
}

#[proc_macro]
pub fn mappings_sig(input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as LitStr);
    let result = convert_sig(&sig.value());

    quote::quote! {
        #result
    }
    .into()
}

struct RegisterBlock {
    api: syn::Ident,
    block: syn::Expr,
    name: syn::LitStr,
    t: Option<syn::Ident>,
}
impl Parse for RegisterBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let api = input.parse()?;
        input.parse::<Token![,]>()?;
        let block = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;
        let t = match input.parse::<Option<Token![,]>>()? {
            Some(_) => input.parse()?,
            None => None,
        };

        Ok(Self {
            api,
            block,
            name,
            t,
        })
    }
}

#[proc_macro_error]
#[proc_macro]
pub fn register_block(input: TokenStream) -> TokenStream {
    let register = parse_macro_input!(input as RegisterBlock);
    let api = register.api;
    let name = register.name;
    // default overrided trait is Block
    let t = register.t.unwrap_or(format_ident!("Block"));
    let block = register.block;

    let super_class = match t.to_string().as_str() {
        "Block" => "net/minecraft/block/Block",
        _ => {
            abort!(t.span(), "This argument must be a block trait, found {}", t)
        }
    };

    quote::quote! {
        #api.register_block(#api.register_block_natives(#t::register(&#block), #super_class), #name, #block);
    }
    .into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn block(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_item = parse_macro_input!(item as ItemImpl);
    let trait_name = impl_item
        .trait_
        .clone()
        .map(|v| v.1.segments.last().unwrap().ident.to_string());
    if !trait_name.clone().is_some_and(|v| v == "Block") {
        abort!(
            impl_item.span(),
            "This macro should be called on a Block trait implementation, currently called on {:?}",
            trait_name
        );
    }

    let mappings = get_multiple_class(&[
        "net/minecraft/block/Block",
        "net/minecraft/block/AbstractBlock",
    ]);
    let mut overrided = vec![];
    let mut custom_funcs = vec![];
    for item in &impl_item.items {
        match item {
            ImplItem::Method(method) => {
                if method.sig.ident.to_string() == "register" {
                    abort!(
                        method.sig.ident.span(),
                        "register func should not be overrided !"
                    );
                }
                let rust_method_name = method.sig.ident.to_string();
                let rust_method_name_ident = format_ident!("{}", rust_method_name);
                if let Some(mc_method) =
                    mappings.get_method(&rust_to_java_method(&rust_method_name))
                {
                    let method_name = mc_method.get_java_name();
                    let sig = mc_method.get_java_sig();
                    let mut custom_func_args = vec![];
                    let mut custom_func_args_names = vec![];
                    let mut custom_func_body = quote! {};
                    method
                        .sig
                        .inputs
                        .iter()
                        .skip(1) // Skip self and api
                        .enumerate()
                        .for_each(|(i, arg)| match arg {
                            syn::FnArg::Typed(arg) => {
                                let type_name = &arg.ty;
                                let arg_name = format_ident!("var{}", i);
                                custom_func_args.push(quote! {
                                    #arg_name: rustcraft::prelude::JObject<'local>
                                });
                                custom_func_args_names.push(quote! {
                                    &mut #arg_name
                                });
                                custom_func_body = quote! {
                                    #custom_func_body
                                    let mut #arg_name = rustcraft::net::minecraft::world::World::new(api.clone(), #arg_name);
                                    // let #arg_name = #type_name::new(api.clone(), #arg_name);
                                }
                            }
                            _ => {}
                        });
                    let has_return = match &method.sig.output {
                        syn::ReturnType::Default => false,
                        syn::ReturnType::Type(_, _) => true,
                    };
                    let (ret_t, ret) = if has_return {
                        (
                            quote! {
                                -> rustcraft::prelude::JObject<'local>
                            },
                            quote! {
                                unsafe { JObject::from_raw(rustcraft::object::ToJava::to_java(result, api.clone()).clone()) }
                            },
                        )
                    } else {
                        (quote! {}, quote! {})
                    };
                    let custom_ident = format_ident!("__rustcraft_{}", rust_method_name_ident);
                    custom_funcs.push(quote! {
                        pub extern "system" fn #custom_ident<
                            'local,
                        >(
                            env: rustcraft::prelude::JNIEnv<'local>,
                            obj: rustcraft::prelude::JObject<'local>,
                            #(#custom_func_args),*
                        ) #ret_t {
                            let mut api = ModApi::new(env, obj);
                            #custom_func_body

                            let block: &mut Box<dyn rustcraft::block::Block> = rustcraft::prelude::load_object(api.clone(), None);

                            let result = block.#rust_method_name_ident(#(#custom_func_args_names),*);
                            #ret
                        }
                    });
                    overrided.push(
                        quote! {(#method_name, #sig, #custom_ident as *mut std::ffi::c_void)},
                    );
                }
            }
            _ => {}
        }
    }

    let new_method = quote! {
        fn register(&self) -> Vec<(&'static str, &'static str, *mut std::ffi::c_void)> {
            vec![#(#overrided),*]
        }
    }
    .into();
    impl_item
        .items
        .push(ImplItem::Method(ImplItemMethod::from(parse_macro_input!(
            new_method as ImplItemMethod
        ))));

    quote::quote! {
        #(#custom_funcs)*
        #impl_item
    }
    .into()
}

#[proc_macro_error]
#[proc_macro]
pub fn auto_gen(_item: TokenStream) -> TokenStream {
    auto_gen_impl().into()
}

#[proc_macro_error]
#[proc_macro]
pub fn auto_gen_to_file(_item: TokenStream) -> TokenStream {
    let result = auto_gen_impl();
    // let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let current_dir = Path::new("/home/julien/code/rustCraft/rustcraft/src");
    println!("Writted in {:?}", current_dir);
    fs::write(current_dir.join("test.rs"), result.to_string()).unwrap();

    quote!().into()
}
