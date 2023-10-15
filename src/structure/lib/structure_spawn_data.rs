use crate::structure::behaivour::logic::lib::sb_script::SBScript;

use super::structure::Structure;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StructureSpawnData {
    pub structure: Structure,
    pub script: SBScript,
}
