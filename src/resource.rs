use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelData {
    pub block_gird: [[u8; 30]; 30]
}
impl Default for LevelData {
    fn default() -> Self {
        LevelData { block_gird: [[0; 30]; 30] }
    }
}