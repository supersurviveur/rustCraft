use std::{
    collections::HashMap,
    env,
    fmt::Write,
    fs::{self, DirEntry, File},
    io::{self, Read},
    iter::Peekable,
    path::Path,
    str::Lines,
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};

static MAPPINGS: LazyLock<Mappings> = LazyLock::new(|| parse_mappings().into());

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Class {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub comments: String,
    pub fields: HashMap<String, Field>,
    pub methods_nosig: HashMap<String, String>,
    methods: HashMap<String, Method>,
    pub inner_classes: HashMap<String, Class>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub field_type: String,
    pub mapped_field_type: String,
    pub comments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Method {
    pub intermediary_name: String,
    pub mapped_name: String,
    pub signature: String,
    pub mapped_signature: String,
    pub comments: String,
    pub args: Vec<Arg>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arg {
    pub position: u16,
    pub name: String,
    pub comment: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Mappings {
    pub(crate) mapped_map: HashMap<String, Class>,
    pub(crate) intermediary_map: HashMap<String, String>,
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
    tmp.map(|s| s.get(0..1).unwrap().to_uppercase() + s.get(1..).unwrap())
        .collect()
}

pub fn method_to_java_convention(s: &str) -> String {
    to_rust_convention(s)
}

pub fn rust_to_java_method(s: &str) -> String {
    let s = s.to_lowercase();
    let mut tmp = s.split("_");
    let mut result = tmp.next().unwrap().to_owned();
    result.extend(tmp.map(|s| s.get(0..1).unwrap().to_uppercase() + s.get(1..).unwrap()));
    result
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
        inner_classes: HashMap::new(),
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

fn parse_mappings() -> Mappings {
    // Check if a serialized file exists
    let current_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if let Ok(cache) = fs::read(current_dir.join("mappings_cache.bin")) {
        return bincode::deserialize(&cache).unwrap();
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
    fs::write(
        current_dir.join("mappings_cache.bin"),
        bincode::serialize(&mappings).unwrap(),
    )
    .unwrap();

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
