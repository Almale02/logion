use std::{usize};

use bevy::utils::HashMap;
use regex::Regex;



#[derive(Debug, Clone, Default)]
pub struct NestingTree {
    pub node_pool: HashMap<String, NTNodeData>,
    pub chidren_data: HashMap<String, HashMap<String, NTNodeData>>,
    pub pointer: Vec<String>,
}

impl NestingTree {
    pub fn add_node(&mut self, path: NTPath) {
        for (idx, dir) in path.directories.iter().enumerate() {
            let path_slice = &path.joint_parts(idx + 1);
            if self
                .node_pool
                .get(path_slice.into_string().as_str())
                .is_none()
            {
                // SECTION: add to chidren data
                self.chidren_data
                    .insert(path_slice.into_string(), HashMap::default());
                if idx > 0 {
                    self.chidren_data
                        .get_mut(&path.joint_parts(idx).into_string())
                        .unwrap()
                        .insert(dir.into(), NTNodeData::dir(dir.into()));
                }

                // SECTION: add to node_pool
                self.node_pool.insert(
                    path.joint_parts(idx + 1).into_string(),
                    NTNodeData {
                        value: dir.into(),
                        endpoint: false,
                    },
                );
            }
        }
        self.node_pool.insert(
            path.into_string(),
            NTNodeData {
                value: path.endpoint.clone(),
                endpoint: true,
            },
        );
        self.chidren_data
            .get_mut(&path.joint_parts(path.directories.len()).into_string())
            .unwrap()
            .insert(
                path.endpoint.clone(),
                NTNodeData::endpoint(path.endpoint.clone()),
            );
    }
    pub fn down_to(&mut self, dir: String) -> HashMap<String, NTNodeData> {
        self.pointer.push(dir);

        return self
            .chidren_data
            .get(&NTPath::from_parts(self.pointer.clone(), "".into()).into_string())
            .unwrap()
            .clone();
    }
    pub fn up(&mut self) -> HashMap<String, NTNodeData> {
        let prev = self.pointer.clone();
        if self.pointer.len() == 1 {
            return self
                .chidren_data
                .get(&NTPath::from_parts(prev, "".into()).into_string())
                .unwrap()
                .clone();
        }
        self.pointer.pop();

        return self
            .chidren_data
            .get(&NTPath::from_parts(self.pointer.clone(), "".into()).into_string())
            .unwrap()
            .clone();
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NTNodeData {
    pub value: String,
    pub endpoint: bool,
}
impl NTNodeData {
    pub fn dir(value: String) -> NTNodeData {
        NTNodeData {
            value,
            endpoint: false,
        }
    }
    pub fn endpoint(value: String) -> NTNodeData {
        NTNodeData {
            value,
            endpoint: true,
        }
    }
}
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NTPath {
    pub directories: Vec<String>,
    pub endpoint: String,
}
impl NTPath {
    pub fn from_parts(directories: Vec<String>, endpoint: String) -> NTPath {
        NTPath {
            directories,
            endpoint,
        }
    }
    pub fn from_string(string: String) -> NTPath {
        let parts = Regex::new(r"([^:]+)").unwrap();
        let parts = parts.captures_iter(&string);

        let mut parts: Vec<_> = parts
            .into_iter()
            .map(|x| x.get(1).unwrap().as_str().to_string())
            .collect();
        let endpoint = parts.pop().unwrap();

        NTPath {
            directories: parts,
            endpoint,
        }
    }
    pub fn into_string(&self) -> String {
        let mut out = String::default();

        for part in self.directories.clone() {
            out.push_str(&(part + ":"))
        }
        if self.endpoint == "".to_string() {
            out.pop();
        }
        out.push_str(self.endpoint.as_str());
        out
    }
    pub fn joint_parts(&self, len: usize) -> NTPath {
        let mut out = String::default();
        let mut parts = self.directories.clone();

        parts.push(self.endpoint.clone());
        let slice = &parts[0..len];

        for (idx, part) in slice.iter().enumerate() {
            let mut to_push = String::default();
            to_push.push_str(part.as_str());

            if idx != slice.len() - 1 {
                to_push.push(':');
            }
            out.push_str(to_push.as_str());
        }

        NTPath::from_string(out)
    }
}

/*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*/
