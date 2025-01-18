use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

pub fn enum_to_java_impl(attr: TokenStream, mut input: ItemEnum) -> TokenStream {
    // Parse attr to get name of the java class
    let class_name = parse_macro_input!(attr as syn::Lit);
    let name = &input.ident;

    let mut match_cases = proc_macro2::TokenStream::new();

    // If the enum is empty, return an empty match
    if input.variants.is_empty() {
        return quote::quote! {#input}.into();
    }

    for variant in &input.variants {
        let variant = &variant.ident;
        // let variant_name = variant.to_string();
        let variant_name = "field_5812".to_string();
        match_cases.extend(quote::quote! {
            #name::#variant => {
                let enum_class = api.current_env.find_class(#class_name).expect(&format!("Couldn't find class {}", #class_name));
                api.current_env.get_static_field(enum_class, #variant_name, &format!("L{};", #class_name)).expect(&format!("Couldn't get field {}", #variant_name))
            }
        });
    }

    let output = quote::quote! {
        #[allow(non_camel_case_types)]
        #input
        impl #name {
            pub fn to_java<'local>(&self, api: &mut ModApi<'local>) -> JObject<'local> {
                let enum_field = match self {
                    #match_cases
                };
                enum_field.l().unwrap()
            }
        }
    };

    output.into()
}
