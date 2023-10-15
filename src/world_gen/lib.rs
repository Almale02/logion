use noise::Fbm;

#[derive(Debug, Clone)]
pub struct WorldGenData {
    pub seed: u32,
    pub perlin: Fbm<noise::Perlin>,
    pub terrain_height: TerrainHeight,
}
impl Default for WorldGenData {
    fn default() -> Self {
        let mut seed = rand::random::<u32>();
        seed = 3;

        WorldGenData {
            seed,
            perlin: Fbm::new(seed),
            terrain_height: TerrainHeight {
                //terrain_height_smoothness: f64::MAX,
                terrain_height_smoothness: 270.,
                perlin_height_multiplier: 30.,
                perlin_height_increment: 0.8,
            },
        }
    }
}
#[derive(Clone, Debug)]
pub struct TerrainHeight {
    pub terrain_height_smoothness: f64,
    pub perlin_height_multiplier: f64,
    pub perlin_height_increment: f64,
}
