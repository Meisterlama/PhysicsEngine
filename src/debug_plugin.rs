use std::cmp::{max, min};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_inspector_egui::egui::Event::Key;

use crate::{DoNotDestroy, MainCamera, random_poly};
use crate::polygon_component::PolygonComponent;
use crate::random_poly::RandomPolyConfig;

pub struct DebugPlugin;

fn add_polygons(mut commands: Commands, keys: Res<Input<KeyCode>>,
                query: Query<Entity, (With<PolygonComponent>, Without<DoNotDestroy>)>,
                q_camera: Query<(&Transform, &Projection), With<MainCamera>>,
)
{
    let (transform, proj) = q_camera.single();
    let mut config = RandomPolyConfig::default();

    let scale = match proj {
        Projection::Perspective(proj) => {1f32}
        Projection::Orthographic(proj) => {proj.scale}
    };

    config.min_bounds *= scale;
    config.max_bounds *= scale;
    config.min_bounds += transform.translation.xy();
    config.max_bounds += transform.translation.xy();

    config.min_radius = min((config.min_radius as f32 * scale) as i32, 1);
    config.max_radius = max((config.max_radius as f32 * scale) as i32, 2);

    if keys.just_pressed(KeyCode::Z) || keys.pressed(KeyCode::X)
    {
        for _ in 0..100
        {
            commands.spawn(random_poly::create_random_poly(&config));
        }
    }

    if keys.just_pressed(KeyCode::F1)
    {
        for entity in query.iter()
        {
            commands.entity(entity).despawn();
        }
    }

    if keys.just_pressed(KeyCode::C) {
        commands.spawn(random_poly::create_square(100f32, 20f32, Vec2::ZERO, 0f32));
    }
}

fn update_fps(diag: Res<Diagnostics>, mut windows: ResMut<Windows>)
{
    let wnd = windows.get_primary_mut().unwrap();

    if let Some(fps_diag) = diag.get(FrameTimeDiagnosticsPlugin::FPS)
    {
        if let Some(fps) = fps_diag.smoothed()
        {
            wnd.set_title(format!("FPS: {:.0}", fps));
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_polygons);
        app.add_system(update_fps);
    }
}