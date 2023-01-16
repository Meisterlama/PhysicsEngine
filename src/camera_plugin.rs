use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

use crate::MainCamera;

pub struct CameraControllerPlugin;

fn zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Projection), With<MainCamera>>,
) {
    let (_transform, mut projection) = query.single_mut();

    let Projection::Orthographic(ref mut proj) = *projection else { todo!() };

    for ev in scroll_evr.iter() {
        let mut log_scale = proj.scale.ln();
        log_scale -= 10f32 * ev.y * time.delta_seconds();

        proj.scale = log_scale.exp();
    }
}

fn move_camera(
    kb_buttons: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    m_buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
)
{
    let (mut transform, projection) = query.single_mut();

    let Projection::Orthographic(ref projection) = *projection else { todo!() };

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

/// This Plugin handle the moving and zooming of the camera in worldspace. If the projection is
/// orthographic, the zoom is logarithmically scaled whereas if the projection is perspective,
/// only the translation is changed
impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(zoom);
        app.add_system(move_camera);
    }
}