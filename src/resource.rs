use bevy::prelude::*;

#[derive(Resource)]
pub struct LevelData {
    pub grid_unit: u8,
    pub world_size: UVec2,
    pub block_gird: Vec<Vec<u8>>
}
impl Default for LevelData {
    fn default() -> Self {        
        LevelData {
            grid_unit: 16,
            world_size: UVec2 {x: 90, y: 30},
            block_gird: vec![vec![0; 90]; 30]
        }
    }
}
impl LevelData {
    pub fn grid_to_global_space_unit(&self, grid_pos: UVec2) -> Vec2{
        let unit_global_width_half = self.world_size.x as i32 /2 as i32;
        let unit_global_height_half = self.world_size.x as i32 /2 as i32;
        
        return Vec2 {
            x: -unit_global_width_half as f32 + grid_pos.x as f32,
            y: unit_global_height_half as f32 - grid_pos.y as f32,
        }
    }

    pub fn loop_block_grid_mut<T>(&mut self, func: T) where T: Fn(usize, usize, &mut u8) {
        for (y, row) in self.block_gird.iter_mut().enumerate() {
            for (x, block) in row.iter_mut().enumerate() {
                func(x, y, block)
            }
        }
    }

    pub fn loop_block_grid<T>(&self, func: T) where T: Fn(usize, usize, &u8) {
        for (y, row) in self.block_gird.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                func(x, y, block)
            }
        }
    }
}