use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::Vec2Swizzles;
use bevy::prelude::*;

use crate::MainCamera;

pub struct CameraControllerPlugin;

fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = query.single_mut();

    for ev in scroll_evr.iter() {
        let mut log_scale = projection.scale.ln();
        log_scale -= 10f32 * ev.y * time.delta_seconds();

        projection.scale = log_scale.exp();
    }
}

fn move_camera(
    kb_buttons: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    m_buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
)
{
    let (mut transform, projection) = query.single_mut();

    for ev in motion_evr.iter() {
        if m_buttons.pressed(MouseButton::Middle)
        {
            let mut delta = ev.delta * projection.scale;
            delta.x *= -1f32;
            transform.translation += delta.extend(0f32);
        }
    }

    let offset = 100f32 * projection.scale * time.delta_seconds();

    if kb_buttons.pressed(KeyCode::W)
    {
        transform.translation.y += offset;
    } else if kb_buttons.pressed(KeyCode::S)
    {
        transform.translation.y -= offset;
    }
    if kb_buttons.pressed(KeyCode::D)
    {
        transform.translation.x += offset;
    } else if kb_buttons.pressed(KeyCode::A)
    {
        transform.translation.x -= offset;
    }
}

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(zoom);
        app.add_system(move_camera);
    }
}