//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::prelude::*;
use bevy_mod_billboard::{
    plugin::BillboardPlugin, BillboardLockAxis, BillboardLockAxisBundle, BillboardTextBundle,
};

use crate::cell::Cell;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BillboardPlugin)
        .insert_resource(AmbientLight {
            brightness: f32::MAX,
            ..default()
        })
        .add_systems(Update, spawn_cell_labels);
}

fn spawn_cell_labels(mut commands: Commands, cells: Query<(Entity, &Cell), Changed<Cell>>) {
    for (entity, cell) in cells.iter() {
        let label = commands
            .spawn((
                Name::from("Label"),
                BillboardLockAxisBundle {
                    billboard_bundle: BillboardTextBundle {
                        transform: Transform::from_translation(Vec3::new(0., 0.1, 0.))
                            .with_scale(Vec3::splat(0.05))
                            .looking_to(Vec3::Y, Vec3::NEG_Z),
                        text: Text::from_section(
                            format!("{}", cell.coordinates()),
                            TextStyle {
                                color: Color::BLACK,
                                font_size: 72.,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        ..default()
                    },
                    lock_axis: BillboardLockAxis {
                        y_axis: true,
                        rotation: true,
                    },
                },
            ))
            .id();
        commands.entity(entity).replace_children(&[label]);
    }
}
