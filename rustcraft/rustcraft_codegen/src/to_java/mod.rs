use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

pub fn enum_to_java_impl(attr: TokenStream, input: ItemEnum) -> TokenStream {
    // Parse attr to get name of the java class
    let class_name = parse_macro_input!(attr as syn::LitStr);
    let binding = class_name.value();

    let mappings = rustcraft_mappings::get_class(&binding);
    let class_name = mappings.get_java_name();

    let name = &input.ident;

    let mut match_cases = proc_macro2::TokenStream::new();

    for variant in &input.variants {
        let variant = &variant.ident;
        let variant_name = variant.to_string();
        let field = mappings
            .fields
            .get(&variant_name)
            .expect(format!("Variant {} doesn't exists", variant_name).as_str());

        let variant_name = field.get_java_name();
        let type_name = field.get_java_type();
        match_cases.extend(quote::quote! {
            #name::#variant => {
                api.get_static_field(&enum_class, (#variant_name, #type_name))
            }
        });
    }

    let output = quote::quote! {
        #input
        impl crate::object::ToJava for #name {
            fn to_java<'local>(self, api: &mut ModApi<'local>) -> JObject<'local> {
                let enum_class = api.get_class(#class_name);
                let enum_field = match self {
                    #match_cases
                };
                enum_field.l().unwrap()
            }
        }
    };

    output.into()
}
