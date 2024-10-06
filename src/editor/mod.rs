mod label;
mod selection;
mod ui;

#[cfg(feature = "dev_native")]
mod wireframe;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((label::plugin, ui::plugin, selection::plugin));

    #[cfg(feature = "dev_native")]
    {
        app.add_plugins(wireframe::plugin);
    }
}
