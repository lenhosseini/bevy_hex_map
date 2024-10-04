mod mesh;

pub use mesh::*;

use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};

use crate::grid::Coordinates;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Cell>();
}

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Cell(Coordinates);

impl Cell {
    pub fn new(coordinates: Coordinates) -> Self {
        Self(coordinates)
    }

    pub fn coordinates(&self) -> Coordinates {
        self.0
    }
}

#[derive(Clone, Bundle, Default)]
pub struct CellBundle {
    pub name: Name,
    pub cell: Cell,
    pub pbr: PbrBundle,
}

#[derive(Debug)]
pub struct SpawnCell {
    pub hexagon: Hexagon,
    pub translation: Vec3,
    pub coordinates: Coordinates,
}

impl Command for SpawnCell {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_cell);
    }
}

fn spawn_cell(
    In(config): In<SpawnCell>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(CellBundle {
        name: Name::from("Cell"),
        cell: Cell::new(config.coordinates),
        pbr: PbrBundle {
            mesh: meshes.add(config.hexagon.mesh()).into(),
            material: materials.add(Color::WHITE).into(),
            transform: Transform::from_translation(config.translation),
            ..default()
        },
    });
}
