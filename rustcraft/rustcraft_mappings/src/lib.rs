use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{
    collections::HashMap,
    env,
    fmt::Write,
    fs::{self, DirEntry, File},
    io::{self, Read},
    iter::Peekable,
    ops::BitAnd,
    path::Path,
    str::{Chars, Lines},
    sync::LazyLock,
};
pub mod codegen;

use bitcode::{Decode, Encode};

pub static MAPPINGS: LazyLock<Mappings> = LazyLock::new(|| parse_mappings().into());

#[derive(Encode, Decode, Debug, Clone)]
pub enum Modifier {
    None = 0,
    Static = 1,
    Nullable = 2,
}

impl BitAnd<u8> for Modifier {
    type Output = bool;

    fn bitand(self, rhs: u8) -> Self::Output {
        (self as u8) & rhs != 0
    }
}

impl BitAnd<Modifier> for u8 {
    type Output = bool;

    fn bitand(self, rhs: Modifier) -> Self::Output {
        rhs & self
    }
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Class {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub comments: String,
    pub fields: HashMap<String, Field>,
    pub methods_nosig: HashMap<String, String>,
    pub methods: HashMap<String, Method>,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Field {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub field_type: String,
    pub mapped_field_type: String,
    pub comments: String,
    pub modifiers: u8,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Method {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub signature: String,
    pub mapped_signature: String,
    pub comments: String,
    pub args: Vec<Arg>,
    pub modifiers: u8,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Arg {
    pub position: u16,
    pub name: String,
    pub comment: String,
    pub modifiers: u8,
}
pub fn is_dev() -> bool {
    std::env::var("DEV_MAPPINGS").is_ok_and(|x| x == "1")
}

impl Field {
    pub fn get_java_name(&self) -> &str {
        if is_dev() {
            &self.mapped_name
        } else {
            &self.intermediary_name
        }
    }
    pub fn get_java_type(&self) -> &str {
        if is_dev() {
            &self.mapped_field_type
        } else {
            &self.field_type
        }
    }
}

impl Method {
    pub fn get_java_name(&self) -> &str {
        if is_dev() {
            &self.mapped_name
        } else {
            &self.intermediary_name
        }
    }
    pub fn get_java_sig(&self) -> &str {
        if is_dev() {
            &self.mapped_signature
        } else {
            &self.signature
        }
    }
}

impl Class {
    pub fn get_java_name(&self) -> &str {
        if is_dev() {
            &self.mapped_name
        } else {
            &self.intermediary_name
        }
    }
    pub fn get_method(&self, method: &str) -> Option<&Method> {
        self.methods.get(
            self.methods_nosig
                .get(method)
                .unwrap_or(&method.to_string()),
        )
    }
}

#[derive(Encode, Decode, Debug)]
pub struct Mappings {
    pub mapped_map: HashMap<String, Class>,
    pub intermediary_map: HashMap<String, String>,
}

impl Mappings {
    pub fn new() -> Self {
        Self {
            mapped_map: HashMap::new(),
            intermediary_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, mapped_name: String, intermediary_name: String) {
        self.intermediary_map.insert(intermediary_name, mapped_name);
    }

    pub fn insert_class(&mut self, mapped_name: String, intermediary_name: String, class: Class) {
        self.mapped_map.insert(mapped_name.clone(), class);
        self.insert(mapped_name, intermediary_name);
    }

    pub fn get(&self, name: &str) -> &Class {
        match self.mapped_map.get(name) {
            None => self
                .mapped_map
                .get(self.intermediary_map.get(name).expect(
                    format!("Can't find class {} in intermediary and mapped name", name).as_str(),
                ))
                .unwrap(),
            Some(value) => value,
        }
    }
}

fn to_rust_convention(s: &str) -> String {
    let s = s.to_lowercase();
    let tmp = s.split("_");
    let result = tmp
        .map(|s| s.get(0..1).unwrap().to_uppercase() + s.get(1..).unwrap())
        .collect();
    result
}

pub fn method_to_java_convention(s: &str) -> String {
    to_rust_convention(s)
}

pub fn normalize(s: &mut String, insert: &str) -> bool {
    match s.as_str() {
        "type" | "match" | "move" | "use" | "self" | "in" | "where" | "macro" | "impl" | "box"
        | "mod" | "ref" | "as" | "true" | "false" | "continue" => {
            s.insert_str(0, insert);
            return true;
        }
        _ => {}
    }
    if s.chars().next().unwrap().is_numeric() {
        s.insert_str(0, insert);
        return true;
    }
    false
}
pub fn rust_to_java_method(s: &str) -> String {
    let s = s.to_lowercase();
    let mut tmp = s.split("_");
    let mut result = tmp.next().unwrap().to_owned();
    result.extend(tmp.map(|s| s.get(0..1).unwrap().to_uppercase() + s.get(1..).unwrap()));
    result
}

pub fn java_to_rust_class(s: &str) -> String {
    let mut result = s.to_string();
    normalize(&mut result, "Class");
    result
}

pub fn java_to_rust_method(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            // Ajoute un underscore avant la majuscule, sauf au début
            if i != 0 {
                result.push('_');
            }
            // Convertit la majuscule en minuscule
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    normalize(&mut result, "method_");
    result
}

pub fn java_to_rust_package(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            // Ajoute un underscore avant la majuscule, sauf au début
            if i != 0 {
                result.push('_');
            }
            // Convertit la majuscule en minuscule
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    normalize(&mut result, "package_");
    result
}

pub fn java_to_rust_field(s: &str) -> String {
    if s.chars().next().unwrap().is_uppercase() {
        s.to_string().to_ascii_lowercase()
    } else {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                // Ajoute un underscore avant la majuscule, sauf au début
                if i != 0 {
                    result.push('_');
                }
                // Convertit la majuscule en minuscule
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }
        result
    }
}

pub enum SigType {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Void,

    Array(Box<SigType>),
    Object(String),
}
impl SigType {
    pub fn jni_name(&self) -> char {
        match self {
            SigType::Boolean => 'z',
            SigType::Byte => 'b',
            SigType::Char => 'c',
            SigType::Short => 's',
            SigType::Int => 'i',
            SigType::Long => 'j',
            SigType::Float => 'f',
            SigType::Double => 'd',
            SigType::Void => 'v',
            SigType::Array(_) => 'l',
            SigType::Object(_) => 'l',
        }
    }
}

pub struct Signature {
    pub ret: SigType,
    pub args: Vec<SigType>,
}

impl SigType {
    pub fn get_constructor(&self) -> TokenStream {
        match self {
            SigType::Array(_) => {
                quote! { unreachable!() }
            }
            SigType::Object(s) => {
                let mut t = vec![];
                let length = s.split('/').count();
                for (i, pat) in s.split('/').enumerate() {
                    let tmp = pat.split('$');
                    let last = tmp.clone().last().unwrap();
                    let l = pat.chars().filter(|c| *c == '$').count();
                    if l >= 1 {
                        for pat in tmp.take(l) {
                            t.push(format_ident!("{}", java_to_rust_package(pat)));
                        }
                    }
                    t.push(format_ident!(
                        "{}",
                        if i == length - 1 {
                            java_to_rust_class(last)
                        } else {
                            java_to_rust_package(last)
                        }
                    ));
                }
                quote! {crate::#(#t)::*}
            }
            _ => unreachable!(),
        }
        .into()
    }
}
impl quote::ToTokens for SigType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend::<TokenStream>(
            match self {
                SigType::Boolean => quote! {bool},
                SigType::Long => quote! {i64},
                SigType::Byte => quote!(i8),
                SigType::Char => quote!(u16),
                SigType::Short => quote!(i16),
                SigType::Int => quote!(i32),
                SigType::Float => quote!(f32),
                SigType::Double => quote!(f64),
                SigType::Void => quote! {()},
                SigType::Array(sig_type) => {
                    let mut t = TokenStream::new();
                    sig_type.to_tokens(&mut t);
                    quote! {Vec<#t>}
                }
                SigType::Object(s) => {
                    let mut t = vec![];
                    let length = s.split('/').count();
                    for (i, pat) in s.split('/').enumerate() {
                        let tmp = pat.split('$');
                        let last = tmp.clone().last().unwrap();
                        let l = pat.chars().filter(|c| *c == '$').count();
                        if l >= 1 {
                            for pat in tmp.take(l) {
                                t.push(format_ident!("{}", java_to_rust_package(pat)));
                            }
                        }
                        t.push(format_ident!(
                            "{}",
                            if i == length - 1 {
                                java_to_rust_class(last)
                            } else {
                                java_to_rust_package(last)
                            }
                        ));
                    }
                    quote! {crate::#(#t)::*<'a>}
                }
            }
            .into(),
        );
    }
}

impl Method {
    pub fn to_tokens(&self, class_name: &str, _struct_name: &Ident) -> TokenStream {
        let mut tokens = quote!();
        let method_name = java_to_rust_method(&self.mapped_name);

        let method_type = parse_java_signature(&self.mapped_signature);
        let jni_method_type = format_ident!("{}", method_type.ret.jni_name());
        let method_ret = method_type.ret;
        let method_sig = self.get_java_sig();
        let method_java_name = self.get_java_name();

        // TODO avoid some skip, currently not supported
        match &method_ret {
            SigType::Object(o) => {
                if o.contains("com/mojang")
                    || o.contains("java/")
                    || o.contains("org/")
                    || o.contains("javax/")
                    || o.contains("google/")
                    || o.contains("it/")
                    || o.contains("io/")
                    || o.contains("jcraft/")
                    || o.contains("ibm/")
                    || o.contains("microsoft/")
                    || o.contains("sun/")
                {
                    return quote!();
                }
            }
            SigType::Array(_) => return quote!(),
            _ => {}
        }

        // Wrap constructors
        if self.mapped_name == "<init>" {
            return quote!();
        } else if self.mapped_name.contains("lambda$") {
            // Lambda inside function, skip
            return quote!();
        }

        let method_ident = format_ident!("{}", method_name);
        let mut method_content = quote!();
        let args;
        if self.modifiers & Modifier::Static {
            args = quote! {api: &'a mut crate::api::ModApi<'a>};
            method_content.extend::<TokenStream>(
                quote! {
                        let class = api.get_class(#class_name);
                        let value =
                             api
                             .call_static_method(
                                 &class,
                                 (#method_java_name, #method_sig),
                                 &[],
                             )
                             .#jni_method_type()
                             .unwrap();
                }
                .into(),
            );
        } else {
            args = quote! {&'a self};
            method_content.extend::<TokenStream>(
                quote! {
                        let api = &self.api;
                        let value = api
                             .call_method_class(
                                 Some(#class_name),
                                 Some(&self.inner),
                                 (#method_java_name, #method_sig),
                                 &[],
                             )
                             .#jni_method_type()
                             .unwrap();
                }
                .into(),
            );
        }
        match method_ret {
            SigType::Object(_) => {
                let result_constructor = method_ret.get_constructor();
                method_content.extend(quote! {
                    #result_constructor::new(api.clone(), value)
                })
            }
            _ => method_content.extend(quote! {
                value
            }),
        }
        tokens.extend::<TokenStream>(
            quote! {
                pub fn #method_ident(#args) -> #method_ret {
                    #method_content
                }
            }
            .into(),
        );
        tokens
    }
}
impl Field {
    pub fn to_tokens(&self, class_name: &str, class: &Class) -> TokenStream {
        let mut tokens = quote!();
        let field_name = if self
            .mapped_name
            .chars()
            .next()
            .unwrap()
            .is_ascii_uppercase()
        {
            let mut field_name = self.mapped_name.clone();
            if class.methods_nosig.contains_key(&field_name) || normalize(&mut field_name, "") {
                field_name = format!("{}_FIELD", field_name);
            }
            field_name
        } else {
            let mut field_name = java_to_rust_field(&self.mapped_name);
            if class.methods_nosig.contains_key(&self.mapped_name) || normalize(&mut field_name, "")
            {
                field_name = format!("{}_field", field_name);
            }
            field_name
        };

        let field_type = parse_type(&mut self.mapped_field_type.chars().peekable());
        let field_sig = self.get_java_type();
        let jni_field_type = format_ident!("{}", field_type.jni_name());
        let field_java_name = self.get_java_name();

        // TODO avoid some skip, currently not supported
        match &field_type {
            SigType::Object(o) => {
                if o.contains("com/mojang")
                    || o.contains("java/")
                    || o.contains("org/")
                    || o.contains("javax/")
                    || o.contains("google/")
                    || o.contains("it/")
                    || o.contains("io/")
                    || o.contains("jcraft/")
                    || o.contains("ibm/")
                    || o.contains("microsoft/")
                    || o.contains("sun/")
                    || o.contains("jdk/")
                    || o.contains("oshi/")
                {
                    return quote!();
                }
            }
            SigType::Array(_) => return quote!(),
            _ => {}
        }
        let field_ident = format_ident!("{}", field_name);
        let mut field_content = quote!();
        let args;
        if self.modifiers & Modifier::Static {
            args = quote! {api: &'a mut crate::api::ModApi<'a>};
            field_content.extend::<TokenStream>(
                quote! {
                        let class = api.get_class(#class_name);
                        let value =
                             api
                             .get_static_field(
                                 &class,
                                 (#field_java_name, #field_sig)
                             )
                             .#jni_field_type()
                             .unwrap();
                }
                .into(),
            );
        } else {
            args = quote! {&'a self};
            field_content.extend::<TokenStream>(
                quote! {
                        let api = &self.api;
                        let value = api
                             .get_field_class(
                                 Some(#class_name),
                                 Some(&self.inner),
                                 (#field_java_name, #field_sig)
                             )
                             .#jni_field_type()
                             .unwrap();
                }
                .into(),
            );
        }
        match field_type {
            SigType::Object(_) => {
                let result_constructor = field_type.get_constructor();
                field_content.extend(quote! {
                    #result_constructor::new(api.clone(), value)
                })
            }
            _ => field_content.extend(quote! {
                value
            }),
        }
        tokens.extend::<TokenStream>(
            quote! {
                pub fn #field_ident(#args) -> #field_type {
                    #field_content
                }
            }
            .into(),
        );
        tokens
    }
}
pub fn parse_type(letters: &mut Peekable<Chars>) -> SigType {
    match letters.next().unwrap() {
        'Z' => SigType::Boolean,
        'B' => SigType::Byte,
        'C' => SigType::Char,
        'S' => SigType::Short,
        'I' => SigType::Int,
        'J' => SigType::Long,
        'F' => SigType::Float,
        'D' => SigType::Double,
        'V' => SigType::Void,
        '[' => SigType::Array(Box::new(parse_type(letters))),
        'L' => {
            let mut current_obj = String::new();
            while let Some(c) = letters.next() {
                if c == ';' {
                    break;
                }
                current_obj.push(c);
            }
            SigType::Object(current_obj)
        }
        c => unreachable!("{:?}: {}", letters, c),
    }
}

pub fn parse_java_signature(s: &str) -> Signature {
    let mut letters = s.chars().peekable();
    letters.next();
    let mut args = vec![];
    while Some(')') != letters.peek().copied() {
        args.push(parse_type(&mut letters));
    }
    letters.next();
    let ret = parse_type(&mut letters);
    Signature { ret, args }
}

fn parse_comments(lines: &mut Peekable<Lines>) -> String {
    let mut result = String::new();
    while let Some(line) = lines.peek() {
        if line.contains("COMMENT") {
            result += line.replace("COMMENT", "").trim();
            lines.next();
        } else {
            break;
        }
    }
    result
}

fn parse_method_args(lines: &mut Peekable<Lines>) -> Vec<Arg> {
    let mut result = vec![];
    while let Some(line) = lines.peek() {
        if line.contains("ARG") {
            let line = line.replace("ARG", "");
            let (pos, name) = line.trim().split_once(" ").unwrap();
            lines.next();
            result.push(Arg {
                position: pos.parse::<u16>().unwrap(),
                name: name.to_string(),
                comment: parse_comments(lines),
                modifiers: 0,
            });
        } else {
            break;
        }
    }
    result
}

fn parse_class_inner(
    indent_level: u8,
    parent: Option<&Class>,
    lines: &mut Peekable<Lines>,
    mappings: &mut Mappings,
) {
    let mut line = lines.next().unwrap().split(" ");
    let count = line.clone().count();
    line.next();
    let (mut intermediary_name, mut mapped_name) = if count == 3 {
        (
            line.next().unwrap().to_string(),
            line.next().unwrap().to_string(),
        )
    } else {
        let intermediary = line.next().unwrap().to_string();
        (intermediary.clone(), intermediary)
    };
    if let Some(parent) = parent {
        intermediary_name = parent.intermediary_name.clone() + "$" + &intermediary_name;
        mapped_name = parent.mapped_name.clone() + "$" + &mapped_name;
    }

    let class_comments = parse_comments(lines);

    let mut class = Class {
        intermediary_name,
        mapped_name,
        comments: class_comments,
        fields: HashMap::new(),
        methods: HashMap::new(),
        methods_nosig: HashMap::new(),
    };

    while let Some(line) = lines.peek() {
        // Not in the class anymore
        if !line.starts_with(&"\t".repeat(indent_level.into())) {
            break;
        }
        let (ltype, line) = line.trim().split_once(" ").unwrap();
        match ltype {
            "FIELD" => {
                let mut inner = line.splitn(3, " ");
                let count = inner.clone().count();
                lines.next();

                let intermediary_name = inner.next().unwrap().to_string();
                let mapped_name = if count == 3 {
                    inner.next().unwrap().to_string()
                } else {
                    intermediary_name.clone()
                };

                class.fields.insert(
                    to_rust_convention(&mapped_name),
                    Field {
                        intermediary_name,
                        mapped_name,
                        field_type: inner.next().unwrap().to_string(),
                        mapped_field_type: String::new(),
                        comments: parse_comments(lines),
                        modifiers: 0,
                    },
                );
            }
            "METHOD" => {
                let mut inner = line.splitn(3, " ");
                let count = inner.clone().count();
                lines.next();

                let intermediary_name = inner.next().unwrap().to_string();
                let mapped_name = if count == 3 {
                    inner.next().unwrap().to_string()
                } else {
                    intermediary_name.clone()
                };

                let signature = inner.next().expect(line).to_string();

                let class_sig = format!("{}#{}", mapped_name.clone(), signature);
                if class.methods_nosig.contains_key(&mapped_name) {
                    class
                        .methods_nosig
                        .insert(mapped_name.clone(), "".to_string());
                } else {
                    class
                        .methods_nosig
                        .insert(mapped_name.clone(), class_sig.clone());
                }

                class.methods.insert(
                    class_sig,
                    Method {
                        intermediary_name,
                        mapped_name,
                        signature,
                        mapped_signature: String::new(),
                        comments: parse_comments(lines),
                        args: parse_method_args(lines),
                        modifiers: 0,
                    },
                );
            }
            "CLASS" => {
                parse_class_inner(indent_level + 1, Some(&class), lines, mappings);
            }
            s => unreachable!("unknown pattern: '{}\' with line '{}'", s, line),
        }
    }

    mappings.insert_class(
        class.mapped_name.clone(),
        class.intermediary_name.clone(),
        class,
    );
}

fn visit_dirs<T: FnMut(&DirEntry)>(dir: &Path, cb: &mut T) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn replace_mappings(mappings: &HashMap<String, String>, t: &String) -> String {
    let mut in_type = false;
    let mut tmp_type = String::new();
    let mut result = String::new();
    for letter in t.chars() {
        if in_type {
            if letter == ';' {
                result += mappings.get(&tmp_type).unwrap_or(&tmp_type);

                result.write_char(';').unwrap();
                in_type = false;
            } else {
                tmp_type.write_char(letter).unwrap();
            }
        } else {
            if letter == 'L' {
                in_type = true;
                tmp_type = String::new();
            }
            result.write_char(letter).unwrap();
        }
    }
    result
}

pub fn set_mappings(mappings: &Mappings) {
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    fs::write(
        current_dir.join("mappings_cache.bin"),
        bitcode::encode(mappings),
    )
    .unwrap();
}
pub fn parse_mappings() -> Mappings {
    // Check if a serialized file exists
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if let Ok(cache) = fs::read(current_dir.join("mappings_cache.bin")) {
        return bitcode::decode(&cache).unwrap();
    }
    let mut mappings = Mappings::new();
    let mappings_dir = current_dir.join("yarn/mappings/");
    visit_dirs(&mappings_dir, &mut |dir| {
        let mut file = File::open(dir.path()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        parse_class_inner(1, None, &mut content.lines().peekable(), &mut mappings);
    })
    .unwrap();

    for value in mappings.mapped_map.values_mut() {
        for field in value.fields.values_mut() {
            field.mapped_field_type =
                replace_mappings(&mut mappings.intermediary_map, &field.field_type);
        }

        let mut new_methods = HashMap::new();
        for key in value.methods.keys() {
            let mut method = value.methods.get(key).unwrap().clone();
            method.mapped_signature =
                replace_mappings(&mut mappings.intermediary_map, &method.signature);
            new_methods.insert(
                key.split_once("#")
                    .map(|(start, sig)| {
                        format!(
                            "{}#{}",
                            start,
                            replace_mappings(&mut mappings.intermediary_map, &sig.to_string())
                        )
                    })
                    .unwrap(),
                method,
            );
        }
        value.methods = new_methods;
        for method in value.methods_nosig.values_mut() {
            if !method.is_empty() {
                *method = method
                    .split_once("#")
                    .map(|(start, sig)| {
                        format!(
                            "{}#{}",
                            start,
                            replace_mappings(&mut mappings.intermediary_map, &sig.to_string())
                        )
                    })
                    .unwrap();
            }
        }
    }

    // Add mappings to cache
    set_mappings(&mappings);

    mappings
}

pub fn get_class(class: &str) -> &Class {
    MAPPINGS.get(class)
}

pub struct ClassGetter<'a> {
    classes: Vec<&'a Class>,
}

impl<'a> ClassGetter<'a> {
    pub fn get_method(&self, method: &str) -> Option<&Method> {
        self.classes
            .iter()
            .skip_while(|class| class.get_method(method).is_none())
            .next()
            .map(|class| class.get_method(method).unwrap())
    }
}
pub fn get_multiple_class<'a>(class: &'a [&'a str]) -> ClassGetter<'a> {
    ClassGetter {
        classes: class.iter().map(|class| get_class(class)).collect(),
    }
}

pub fn convert_sig(sig: &str) -> String {
    replace_mappings(&MAPPINGS.intermediary_map, &sig.to_string())
}
