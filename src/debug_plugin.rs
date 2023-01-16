use std::cmp::{max, min};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::{DoNotDestroy, MainCamera, random_poly};
use crate::collision_plugin::plugin::CollisionStage;
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::random_poly::RandomPolyConfig;
use crate::transform2d::Transform2d;

pub struct DebugPlugin;

/// This plugin is responsible for handling keyboard input to add/delete polygons on the screen.
/// It also also handle the automatic killing of entities if they fall to far
/// It update the title of the application to display the FPS
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CollisionStage::PreSyncData,add_polygons);
        app.add_system_to_stage(CollisionStage::PreSyncData, auto_delete_polygons);
        app.add_system(update_fps);
    }
}

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

        // add back the borders that were destroyed (Usage of DoNotDestroy component somehow bug the whole system)
        commands.spawn(random_poly::create_square(100f32, 1000f32, Vec2::new(-1101f32, 0f32), 0f32, true));
        commands.spawn(random_poly::create_square(100f32, 1000f32, Vec2::new(1101f32, 0f32), 0f32, true));
        commands.spawn(random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, 1101f32), 0f32, true));
        commands.spawn(random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, -1101f32), 0f32, true));
    }

    if keys.just_pressed(KeyCode::C) {
        commands.spawn(random_poly::create_square(100f32, 20f32, Vec2::ZERO, 0f32, false));
    }
}

fn auto_delete_polygons(
    query: Query<(Entity, &Transform2d), Changed<Transform2d>>,
    mut command: Commands
) {
    query.for_each(|(e, t)| {
        if t.translation.y < -10000f32 {
            command.entity(e).despawn();
        }
    })
}

fn update_fps(diag: Res<Diagnostics>, mut windows: ResMut<Windows>)
{
    let wnd = windows.get_primary_mut().unwrap();

    if let Some(fps_diag) = diag.get(FrameTimeDiagnosticsPlugin::FPS)
    {
        if let Some(fps) = fps_diag.smoothed()
        {
            wnd.set_title(format!("rusted_physics - FPS: {:.0}", fps));
        }
    }
}