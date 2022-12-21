#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_polyline::prelude::*;

use crate::aabb_update_system::aabb_update_system;
use crate::camera_plugin::CameraControllerPlugin;
use crate::collision_plugin::{CollisionPlugin, CollisionStage};
use crate::debug_plugin::DebugPlugin;
use crate::polygon_plugin::DrawPolygonPlugin;
use crate::polygon_renderer::PolygonRendererPlugin;
use crate::random_poly::RandomPolyConfig;

#[derive(Component)]
struct MainCamera;

mod polygon_component;
mod aabb;
mod polygon_plugin;
mod collision_plugin;
mod random_poly;
mod transform2d;
mod camera_plugin;
mod debug_plugin;
mod aabb_update_system;
mod polygon_renderer;
mod line_renderer;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DoNotDestroy;

fn startup_add_polygons(mut commands: Commands)
{
    let config = RandomPolyConfig::default();

    fastrand::seed(0);
    for _ in 0..30000
    {
        commands.spawn(random_poly::create_random_poly(&config));
    }

    commands.spawn((random_poly::create_square(100f32, 1000f32, Vec2::new(-900f32, 0f32), 0f32), DoNotDestroy));
    commands.spawn((random_poly::create_square(100f32, 1000f32, Vec2::new(900f32, 0f32), 0f32), DoNotDestroy));
    commands.spawn((random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, 500f32), 0f32), DoNotDestroy));
    commands.spawn((random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, -500f32), 0f32), DoNotDestroy));
}

fn add_camera(mut commands: Commands)
{
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 500.0),
        camera: Camera {
            hdr: true,
            ..default()
        },
        projection: Projection::Orthographic(OrthographicProjection::default()),
        ..Camera3dBundle::default()
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
        .add_plugin(PolylinePlugin)
        .add_plugin(DrawPolygonPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(add_camera)
        .add_startup_system(startup_add_polygons)
        .add_system(aabb_update_system)
        .add_system_to_stage(CollisionStage::PostUpdate, line_renderer::render_lines);


        app.add_plugin(PolygonRendererPlugin);


    app.run();
}
