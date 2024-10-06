use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_inspector::ui_for_resource, DefaultInspectorConfigPlugin};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::camera::MainCamera;

use super::label::ShowLabels;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bevy_egui::EguiPlugin, DefaultInspectorConfigPlugin))
        .add_systems(Update, (draw_ui, disable_camera_movement).chain());
}

fn draw_ui(world: &mut World) {
    let mut ctx = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    ctx.get_mut().set_visuals(egui::Visuals {
        window_fill: egui::Color32::from_white_alpha(100),
        ..egui::Visuals::light()
    });

    egui::Window::new("Configuration")
        .collapsible(false)
        .title_bar(false)
        .fixed_pos([20.0, 20.0])
        .default_width(120.)
        .show(ctx.get_mut(), |ui| {
            ui.heading("Settings");

            ui.horizontal(|ui| {
                ui.label("Show Labels");
                ui_for_resource::<ShowLabels>(world, ui);
            });

            #[cfg(feature = "dev_native")]
            {
                use crate::editor::wireframe::ShowWireframes;
                ui.horizontal(|ui| {
                    ui.label("Show Wireframe");
                    ui_for_resource::<ShowWireframes>(world, ui);
                });
            }
        });
}

fn disable_camera_movement(
    mut ctx: Query<&mut bevy_egui::EguiContext, With<PrimaryWindow>>,
    mut cam: Query<&mut PanOrbitCamera, With<MainCamera>>,
) {
    let mut ctx = ctx.single_mut();
    let mut cam = cam.single_mut();
    match ctx.get_mut().is_pointer_over_area() {
        true => {
            cam.enabled = false;
        }
        false => {
            cam.enabled = true;
        }
    };
}
