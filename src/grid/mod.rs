mod coordinates;

pub use coordinates::*;

use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::std_options::NumberDisplay, prelude::*};

use crate::cell::{CellConfig, Hexagon, SpawnCell};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Grid>()
        .register_type::<GridConfig>()
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

#[derive(Debug, Clone, Copy, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GridConfig {
    #[inspector(min = 1, max = 100, display = NumberDisplay::Slider)]
    pub width: u16,
    #[inspector(min = 1, max = 100, display = NumberDisplay::Slider)]
    pub height: u16,
}

impl GridConfig {
    fn cell_translation(&self, width: u16, height: u16, hexagon: Hexagon) -> Vec3 {
        let x = (width as f32 + (height as f32 * 0.5 - (height / 2) as f32))
            * (hexagon.inner_radius() * 2.);
        let z = height as f32 * (hexagon.outer_radius() * 1.5);
        Vec3::new(x, 0., z)
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
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

fn fill_grid(mut commands: Commands, grid_config: Res<GridConfig>, cell_config: Res<CellConfig>) {
    let hexagon = Hexagon::new(cell_config.size);
    for z in 0..grid_config.height {
        for x in 0..grid_config.width {
            commands.add(SpawnCell {
                hexagon: hexagon,
                translation: grid_config.cell_translation(x, z, hexagon),
                coordinates: Coordinates::from_position(x.into(), z.into()),
            })
        }
    }
}
