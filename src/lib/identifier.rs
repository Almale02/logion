use regex::Regex;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Identifier {
    pub id: String,
}
impl Default for Identifier {
    fn default() -> Self {
        Self::new("identifier", "default")
    }
}
impl Identifier {
    pub const STRUCTURE: &str = "struct";
    pub const BUILTIN_DATA_TYPE: &str = "data_dype";
    pub const USER_DATA_TYPE: &str = "user_data_type";
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
}
