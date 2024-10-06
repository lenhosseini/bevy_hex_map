mod mesh;

use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Listener, On},
    PickableBundle,
};
pub use mesh::*;

use bevy::{
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use bevy_inspector_egui::{
    inspector_options::std_options::NumberDisplay, prelude::*, InspectorOptions,
};

use crate::grid::{Coordinates, Grid};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Cell>()
        .register_type::<CellConfig>()
        .init_resource::<CellConfig>();
}

#[derive(Resource, Reflect, Debug, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct CellConfig {
    #[inspector(min = 1.0, max = 10.0, display = NumberDisplay::Slider)]
    pub size: f32,
}

impl Default for CellConfig {
    fn default() -> Self {
        Self { size: 10. }
    }
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

#[derive(Bundle)]
pub struct CellBundle {
    pub name: Name,
    pub cell: Cell,
    pub pbr: PbrBundle,
    pub pickable: PickableBundle,
    pub on_click: On<Pointer<Click>>,
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
    grid: Query<Entity, With<Grid>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell = commands
        .spawn(CellBundle {
            name: Name::from("Cell"),
            cell: Cell::new(config.coordinates),
            pbr: PbrBundle {
                mesh: meshes.add(config.hexagon.mesh()).into(),
                material: materials.add(Color::WHITE).into(),
                transform: Transform::from_translation(config.translation),
                ..default()
            },
            pickable: PickableBundle::default(),
            on_click: On::<Pointer<Click>>::run(on_cell_click),
        })
        .id();

    commands.entity(grid.single()).push_children(&[cell]);
}

pub fn on_cell_click(
    event: Listener<Pointer<Click>>,
    cells: Query<&Handle<StandardMaterial>, With<Cell>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(material) = cells.get(event.target) {
        let material = materials.get_mut(material).unwrap();
        material.base_color = bevy::color::palettes::css::PINK.into();
    }
}
