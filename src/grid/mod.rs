mod coordinates;

pub use coordinates::*;

use bevy::prelude::*;

use crate::cell::{Hexagon, SpawnCell};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Grid>()
        .init_resource::<GridConfig>()
        .add_systems(Startup, (spawn_grid, fill_grid).chain());
}

#[derive(Debug, Clone, Copy, Component, Reflect, Default)]
#[reflect(Component)]
pub struct Grid;

#[derive(Debug, Clone, Bundle, Default)]
pub struct GridBundle {
    pub name: Name,
    pub grid: Grid,
    pub spatial: SpatialBundle,
}

#[derive(Debug, Clone, Copy, Resource, Reflect)]
#[reflect(Resource)]
pub struct GridConfig {
    pub hexagon: Hexagon,
    pub width: u16,
    pub height: u16,
}

impl GridConfig {
    fn cell_translation(&self, width: u16, height: u16) -> Vec3 {
        let x = (width as f32 + (height as f32 * 0.5 - (height / 2) as f32))
            * (self.hexagon.inner_radius() * 2.);
        let z = height as f32 * (self.hexagon.outer_radius() * 1.5);
        Vec3::new(x, 0., z)
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            hexagon: Hexagon::default(),
            width: 6,
            height: 6,
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn(GridBundle {
        name: Name::from("Grid"),
        grid: Grid,
        spatial: SpatialBundle::default(),
    });
}

fn fill_grid(mut commands: Commands, config: Res<GridConfig>) {
    for z in 0..config.height {
        for x in 0..config.width {
            commands.add(SpawnCell {
                hexagon: config.hexagon,
                translation: config.cell_translation(x, z),
                coordinates: Coordinates::from_position(x.into(), z.into()),
            })
        }
    }
}
