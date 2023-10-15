use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::{PrimitiveTopology},
    },
};

pub struct StructTextureHelper {}
impl StructTextureHelper {
    pub fn gen_sprites_from_pos(sprite: Sprite, positions: Vec<Vec3>) -> Vec<(Vec3, Sprite)> {
        let mut out: Vec<(Vec3, Sprite)> = Vec::default();
        for pos in positions {
            out.push((pos, sprite.clone()));
        }
        out
    }
    pub fn gen_meshes_from_pos(
        mesh: Mesh,
        material: ColorMaterial,
        positions: Vec<Vec3>,
    ) -> Vec<(Vec3, Mesh, ColorMaterial)> {
        let mut out: Vec<(Vec3, Mesh, ColorMaterial)> = Vec::default();
        for pos in positions {
            out.push((pos, mesh.clone(), material.clone()));
        }
        out
    }
    pub fn gen_grid_right_triangle_mesh(color: Color) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0., 0., 0.0], [32., 0., 0.0], [32., 32., 0.]],
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![color.as_rgba_f32(); 3]);
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
        mesh
    }
}
