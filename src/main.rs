#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_prototype_debug_lines::*;

use crate::aabb_update_system::aabb_update_system;
use crate::camera_plugin::CameraControllerPlugin;
use crate::collision_plugin::CollisionPlugin;
use crate::debug_plugin::DebugPlugin;
use crate::polygon_plugin::{DrawPoly, DrawPolygonPlugin};
use crate::polygon_renderer::PolygonRendererPlugin;
use crate::random_poly::RandomPolyConfig;

#[derive(Component)]
struct MainCamera;

mod polygon_component;
mod aabb;
mod drawable;
mod polygon_plugin;
mod collision_plugin;
mod random_poly;
mod transform2d;
mod camera_plugin;
mod debug_plugin;
mod aabb_update_system;
mod polygon_renderer;

fn startup_add_polygons(mut commands: Commands)
{
    let config = RandomPolyConfig::default();

    for _ in 0..100
    {
        commands.spawn(random_poly::create_random_poly(&config));
    }
}

fn add_camera(mut commands: Commands)
{
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    }, MainCamera));
}

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "rusted_phyisics".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(DrawPolygonPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(add_camera)
        .add_startup_system(startup_add_polygons)
        .add_system(aabb_update_system);

    // Set to false to use mesh_renderer. It is slower for the moment
    const DRAW_LINE: bool = true;
    if DRAW_LINE
    {
            app.insert_resource(DrawPoly(true));
    }
    else {
        app.add_plugin(PolygonRendererPlugin);
    }

    app.run();
}
