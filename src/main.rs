#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]

extern crate core;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResizeConstraints};
use bevy_polyline::prelude::*;

use collision_plugin::polygon_plugin::DrawPolygonPlugin;

use crate::camera_plugin::CameraControllerPlugin;
use crate::collision_plugin::plugin::{CollisionPlugin};
use crate::debug_plugin::DebugPlugin;
use crate::scene::setup_scene;

#[derive(Component)]
struct MainCamera;

mod collision_plugin;
mod random_poly;
mod transform2d;
mod camera_plugin;
mod debug_plugin;
mod scene;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoNotDestroy;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "rusted_physics".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            resize_constraints: WindowResizeConstraints {
                min_width: 800f32,
                min_height: 800f32,
                ..default()
            },
            ..default()
        },
        ..default()
    }));

    app.add_plugin(FrameTimeDiagnosticsPlugin::default()); // Record smooth Framerate stats
    app.add_plugin(PolylinePlugin); // Plugin to draw line strips
    app.add_plugin(CollisionPlugin); // Main plugin of the crate, responsible for physics
    app.add_plugin(DrawPolygonPlugin); // Batch drawer for polyline
    app.add_plugin(CameraControllerPlugin); //Camera inputs
    app.add_plugin(DebugPlugin); // Various systems such as polygon add/delete and update fps title
    app.add_startup_system(setup_scene); // Adding default scene


    app.run();
}
