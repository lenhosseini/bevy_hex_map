mod camera;
mod cell;
mod grid;

#[cfg(feature = "dev")]
mod dev_tools;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let mut default_plugins = PluginGroupBuilder::start::<DefaultPlugins>();

        default_plugins = default_plugins.add_group(DefaultPlugins);

        default_plugins = default_plugins.set(WindowPlugin {
            primary_window: Window {
                title: "Bevy Hex Map".to_string(),
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }
            .into(),
            ..default()
        });

        #[cfg(feature = "dev_native")]
        {
            use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
            use bevy::render::{settings::*, RenderPlugin};

            default_plugins = default_plugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            });

            default_plugins = default_plugins.add(WireframePlugin);

            app.insert_resource(WireframeConfig {
                global: true,
                default_color: Color::BLACK,
            });
        }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            default_plugins = default_plugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            });
        }

        app.add_plugins((default_plugins, camera::plugin, cell::plugin, grid::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        {
            app.add_plugins(dev_tools::plugin);
        }
    }
}
