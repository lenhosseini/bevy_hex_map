use bevy::{
    math::Vec3,
    prelude::{Mesh, MeshBuilder, Meshable},
    reflect::Reflect,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Hexagon(f32);

impl Default for Hexagon {
    fn default() -> Self {
        Self(10.)
    }
}

impl Hexagon {
    pub fn new(outer_radius: f32) -> Self {
        Self(outer_radius)
    }

    pub fn inner_radius(&self) -> f32 {
        self.outer_radius() * (3_f32.sqrt() / 2_f32)
    }

    pub fn outer_radius(&self) -> f32 {
        self.0
    }

    pub fn vertices(&self) -> impl IntoIterator<Item = Vec3> {
        vec![
            Vec3::new(0., 0., self.outer_radius()), // top center
            Vec3::new(self.inner_radius(), 0., 0.5 * self.outer_radius()), // top right
            Vec3::new(self.inner_radius(), 0., -0.5 * self.outer_radius()), // bottom right
            Vec3::new(0., 0., -self.outer_radius()), // bottom center
            Vec3::new(-self.inner_radius(), 0., -0.5 * self.outer_radius()), // bottom left
            Vec3::new(-self.inner_radius(), 0., 0.5 * self.outer_radius()), // top left
        ]
    }
}

pub struct HexagonMeshBuilder {
    hexagon: Hexagon,
}

impl MeshBuilder for HexagonMeshBuilder {
    fn build(&self) -> bevy::prelude::Mesh {
        let vertices: Vec<_> = self.hexagon.vertices().into_iter().collect();
        // define vertex positions for hexagon with edge vertices and center
        let positions: Vec<Vec3> = vec![vec![Vec3::ZERO], vertices].concat();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone())
        // define triangles of hexagon in counter-clockwise order
        .with_inserted_indices(Indices::U32(vec![
            1, 0, 6, // top left triangle
            6, 0, 5, // center left triangle
            5, 0, 4, // bottom left triangle
            4, 0, 3, // bottom right triangle
            3, 0, 2, // center right triangle
            2, 0, 1, // top right triangle
        ]))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [0.5, 0.5], // center center vertex
                [0.5, 0.],  // top center vertex
                [0., 0.75], // top left vertex
                [0., 0.25], // bottom left vertex
                [0., 0.5],  // bottom center vertex
                [1., 0.25], // bottom right vertex
                [1., 0.75], // top right vertex
            ],
        )
        // define normals for vertices all pointing upwards (positive Y-axis)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::Y; positions.len()])
    }
}

impl Meshable for Hexagon {
    type Output = HexagonMeshBuilder;

    fn mesh(&self) -> Self::Output {
        HexagonMeshBuilder { hexagon: *self }
    }
}

impl From<Hexagon> for Mesh {
    fn from(hexagon: Hexagon) -> Self {
        hexagon.mesh().build()
    }
}
