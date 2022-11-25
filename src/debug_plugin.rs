use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::math::Vec3Swizzles;

use crate::{MainCamera, random_poly};
use crate::polygon_component::PolygonComponent;
use crate::random_poly::RandomPolyConfig;

pub struct DebugPlugin;

fn add_polygons(mut commands: Commands, keys: Res<Input<KeyCode>>,
                query: Query<Entity, With<PolygonComponent>>,
                q_camera: Query<(&Transform, &OrthographicProjection), With<MainCamera>>,
)
{
    let (transform, proj) = q_camera.single();
    let mut config = RandomPolyConfig::default();

    config.min_bounds *= proj.scale;
    config.max_bounds *= proj.scale;
    config.min_bounds += transform.translation.xy();
    config.max_bounds += transform.translation.xy();

    // config.min_radius = (config.min_radius as f32 * proj.scale) as i32;
    // config.max_radius = (config.max_radius as f32 * proj.scale) as i32;

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