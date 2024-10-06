use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ShowWireframes>()
        .init_resource::<ShowWireframes>()
        .add_plugins(WireframePlugin)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::BLACK,
        })
        .add_systems(Update, toggle_wireframes);
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ShowWireframes(bool);

fn toggle_wireframes(
    mut wireframe_config: ResMut<WireframeConfig>,
    show_wireframes: Res<ShowWireframes>,
) {
    if !show_wireframes.is_changed() {
        return;
    }

    wireframe_config.global = show_wireframes.0
}
