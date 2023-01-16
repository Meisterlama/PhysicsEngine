use bevy::prelude::*;
use crate::{MainCamera, random_poly};
use crate::random_poly::RandomPolyConfig;

pub fn setup_scene(mut commands: Commands)
{
    startup_add_polygons(&mut commands);
    add_camera(&mut commands);
}

fn startup_add_polygons(commands: &mut Commands)
{
    fastrand::seed(0);

    let config = RandomPolyConfig::default();

    commands.spawn(random_poly::create_square(100f32, 1000f32, Vec2::new(-1101f32, 0f32), 0f32, true));
    commands.spawn(random_poly::create_square(100f32, 1000f32, Vec2::new(1101f32, 0f32), 0f32, true));
    commands.spawn(random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, 1101f32), 0f32, true));
    commands.spawn(random_poly::create_square(1000f32, 100f32, Vec2::new(0f32, -1101f32), 0f32, true));

    for _ in 0..2000
    {
        // commands.spawn(random_poly::create_square(100f32, 20f32, Vec2::ZERO, 0f32, false));
        commands.spawn(random_poly::create_random_poly(&config));
    }
}

fn add_camera(commands: &mut Commands)
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