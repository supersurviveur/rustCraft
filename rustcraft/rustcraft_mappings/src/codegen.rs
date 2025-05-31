use std::collections::HashMap;

use crate::{java_to_rust_class, java_to_rust_package, Class, MAPPINGS};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct Package {
    pub current: HashMap<String, proc_macro2::TokenStream>,
    pub sub_packages: HashMap<String, Box<Package>>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            current: HashMap::new(),
            sub_packages: HashMap::new(),
        }
    }
    pub fn insert(&mut self, class_name: &str, content: proc_macro2::TokenStream) {
        match class_name.split_once("/").or(class_name.split_once("$")) {
            Some((start, end)) => {
                let start = java_to_rust_package(start);
                if let Some(entry) = self.sub_packages.get_mut(&start) {
                    entry.insert(end, content);
                } else {
                    let mut package = Box::new(Package::new());
                    package.insert(end, content);
                    self.sub_packages.insert(start.to_string(), package);
                }
            }
            None => {
                self.current.insert(class_name.to_string(), content);
            }
        }
    }
    pub fn to_tokens(self) -> proc_macro2::TokenStream {
        let mut sub_packages_content = quote!();
        for (name, sub_package) in self.sub_packages.into_iter() {
            let module_name = format_ident!("{}", name);
            let inner = sub_package.to_tokens();
            sub_packages_content.extend(quote! {
                pub mod #module_name {
                    #inner
                }
            });
        }
        let mut inner = quote!();
        for class in self.current.into_values() {
            inner.extend(class);
        }
        quote! {
            #sub_packages_content
            #inner
        }
    }
}

pub fn auto_gen_impl() -> TokenStream {
    let mut packages = Package::new();
    for class in MAPPINGS.mapped_map.values() {
        packages.insert(&class.mapped_name, gen_class(class));
    }
    let result = packages.to_tokens();
    quote! {
        #![allow(non_snake_case, dead_code)]
        #result
    }
    .into()
}

fn gen_class(mappings: &Class) -> proc_macro2::TokenStream {
    let struct_name = java_to_rust_class(
        &mappings
            .mapped_name
            .rsplit_once("$")
            .unwrap_or(mappings.mapped_name.rsplit_once("/").unwrap())
            .1,
    );
    let struct_name = format_ident!("{}", struct_name);
    let struct_name_interface = format_ident!("{}Interface", struct_name);

    let struct_gen = quote! {
        #[derive(Debug)]
        pub struct #struct_name<'a> {
            pub api: crate::api::ModApi<'a>,
            pub inner: jni::objects::JObject<'a>
        }
    };

    let mut methods = vec![];
    for method in mappings.methods.values() {
        if mappings.methods_nosig.get(&method.mapped_name) == Some(&"".to_string()) {
            // TODO Overloading, skip for the moment
            continue;
        }

        methods.push(method.to_tokens(mappings.get_java_name(), &struct_name));
    }

    for field in mappings.fields.values() {
        methods.push(field.to_tokens(mappings.get_java_name(), mappings));
    }

    let impl_gen = quote! {
        impl<'a> #struct_name<'a> {
            pub fn new(api: crate::api::ModApi<'a>, inner: jni::objects::JObject<'a>) -> Self {
                #struct_name { api, inner }
            }
            #(#methods)*
        }
    };

    quote! {
        #struct_gen
        #impl_gen
    }
}
