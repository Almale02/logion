use regex::Regex;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Identifier {
    pub id: String,
}
impl Default for Identifier {
    fn default() -> Self {
        Self {
            id: "None".to_owned(),
        }
    }
}
impl Identifier {
    pub const STRUCTURE: &str = "struct";
    pub const FUNCTION: &str = "function";
    pub const DATA_TYPE: &str = "data_dype";
    pub const BLOCK: &str = "block";
    pub const MATERIAL: &str = "material";

    pub fn new(base: &str, body: &str) -> Self {
        Identifier {
            id: format!("{base}:{{{body}}}"),
        }
    }
    pub fn get_name(&self) -> String {
        let re = Regex::new(r":(\w+)}").unwrap();
        let caps = re.captures(&self.id).unwrap();
        return caps[1].to_owned();
    }
    pub fn get_body(&self) -> String {
        let re = Regex::new(r"\{(.+)\}").unwrap();
        let caps = re.captures(&self.id).unwrap();
        return caps[1].to_owned();
    }
    pub fn new_data_type(body: &str) -> Self {
        Self::new(Identifier::DATA_TYPE, body)
    }
    pub fn new_function(body: &str) -> Self {
        Self::new(Identifier::FUNCTION, body)
    }
    pub fn new_structure(body: &str) -> Self {
        Self::new(Identifier::STRUCTURE, body)
    }
}
