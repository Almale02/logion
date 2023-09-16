use regex::Regex;
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Identifier {
    pub id: String,
}
impl Identifier {
    pub fn new(id: String) -> Self {
        Identifier { id }
    }
    pub fn get_name(&self) -> String {
        let re = Regex::new(r":(\w+)}").unwrap();
        let caps = re.captures(&self.id).unwrap();
        return caps[1].to_owned();
    }
}
