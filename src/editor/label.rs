use bevy::prelude::*;
use bevy_mod_billboard::{prelude::*, BillboardLockAxis, BillboardLockAxisBundle};

use crate::cell::Cell;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Label>()
        .register_type::<ShowLabels>()
        .init_resource::<ShowLabels>()
        .add_plugins(BillboardPlugin)
        .add_systems(Update, (spawn_cell_labels, toggle_cell_labels));
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ShowLabels(bool);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Label;

fn spawn_cell_labels(
    mut commands: Commands,
    cells: Query<(Entity, &Cell), Changed<Cell>>,
    show_labels: Res<ShowLabels>,
) {
    let visibility = match show_labels.0 {
        true => Visibility::Visible,
        false => Visibility::Hidden,
    };

    for (entity, cell) in cells.iter() {
        let label = commands
            .spawn((
                Name::from("Label"),
                Label,
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
                        visibility,
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

fn toggle_cell_labels(
    mut labels: Query<&mut Visibility, With<Label>>,
    show_labels: Res<ShowLabels>,
) {
    if !show_labels.is_changed() {
        return;
    }

    for mut label in labels.iter_mut() {
        match show_labels.0 {
            true => *label = Visibility::Visible,
            false => *label = Visibility::Hidden,
        };
    }
}
