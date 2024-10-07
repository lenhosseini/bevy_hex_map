mod label;
mod selection;
mod ui;

#[cfg(feature = "dev_native")]
mod wireframe;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<EditorColor>()
        .init_resource::<EditorColor>()
        .add_plugins((label::plugin, ui::plugin, selection::plugin));

    #[cfg(feature = "dev_native")]
    {
        app.add_plugins(wireframe::plugin);
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct EditorColor(pub Color);

impl Default for EditorColor {
    fn default() -> Self {
        Self(bevy::color::palettes::css::HOT_PINK.into())
    }
}
