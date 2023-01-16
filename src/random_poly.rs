use std::default::Default;
use bevy::prelude::*;

use crate::collision_plugin::aabb::AABB;
use crate::collision_plugin::PhysicsAwake;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub struct RandomPolyConfig {
    pub min_points: usize,
    pub max_points: usize,
    pub min_radius: i32,
    pub max_radius: i32,
    pub min_rotation: i32,
    pub max_rotation: i32,
    pub min_linear_speed: Vec2,
    pub max_linear_speed: Vec2,
    pub min_angular_speed: i32,
    pub max_angular_speed: i32,
    pub min_bounds: Vec2,
    pub max_bounds: Vec2,
}

pub fn create_random_poly(config: &RandomPolyConfig) -> (PolygonComponent, Transform2d, AABB, RigidBody2d, PhysicsAwake)
{
    let radius = fastrand::i32(config.min_radius..=config.max_radius);
    let sides = fastrand::usize(config.min_points..=config.max_points);

    let rand_x = {
        let min_x = config.min_bounds.x as i32;
        let max_x = config.max_bounds.x as i32;
        fastrand::i32(min_x..=max_x)
    };
    let rand_y = {
        let min_y = config.min_bounds.y as i32;
        let max_y = config.max_bounds.y as i32;
        fastrand::i32(min_y..=max_y)
    };

    let rotation = fastrand::i32(config.min_rotation..=config.max_rotation) as f32;

    let mut points = Vec::with_capacity(sides.clone());

    let d_angle = 360f32 / sides as f32;

    for i in 0..sides
    {
        let angle = i as f32 * d_angle;

        let point = Vec2::new(angle.to_radians().cos(), angle.to_radians().sin()) * radius as f32;
        points.push(point);
    }

    let position = Vec2::new(rand_x as f32, rand_y as f32);

    let polygon = PolygonComponent::new(points);
    let transform = Transform2d {
        translation: position,
        rotation: rotation,
        scale: 1f32,
    };
    let aabb = AABB::from(&polygon.get_rotated_points(&transform));

    let rand_speed_x = {
        let min_x = config.min_linear_speed.x as i32;
        let max_x = config.max_linear_speed.x as i32;
        fastrand::i32(min_x..=max_x)
    };
    let rand_speed_y = {
        let min_y = config.min_linear_speed.y as i32;
        let max_y = config.max_linear_speed.y as i32;
        fastrand::i32(min_y..=max_y)
    };

    let rand_angular_speed = fastrand::i32(config.min_angular_speed..config.max_angular_speed) as f32;


    let rigidbody = RigidBody2d {
        mass: (radius) as f32,
        is_kinematic: false,
        linear_speed: Vec2::new(rand_speed_x as f32, rand_speed_y as f32),
        angular_speed: rand_angular_speed,
        ..Default::default()
    };

    return (polygon, transform, aabb, rigidbody, PhysicsAwake);
}

pub fn create_square(length: f32, height: f32, position: Vec2, rotation: f32, kinematic: bool) -> (PolygonComponent, Transform2d, AABB, RigidBody2d, PhysicsAwake) {
    let mut points = Vec::with_capacity(4);

    points.push(Vec2::new(-length, -height));
    points.push(Vec2::new(-length, height));
    points.push(Vec2::new(length, height));
    points.push(Vec2::new(length, -height));

    let polygon = PolygonComponent::new(points);
    let transform = Transform2d {
        translation: position,
        rotation: rotation,
        scale: 1f32,
    };
    let aabb = AABB::from(&polygon.get_rotated_points(&transform));
    let mut rigidbody = RigidBody2d::default();
    rigidbody.is_kinematic = kinematic;
    rigidbody.mass = length * height;

    return (polygon, transform, aabb, rigidbody, PhysicsAwake);
}

impl Default for RandomPolyConfig {
    fn default() -> Self {
        RandomPolyConfig {
            min_points: 3,
            max_points: 12,
            min_radius: 10,
            max_radius: 20,
            min_rotation: 0,
            max_rotation: 360,
            min_linear_speed: Vec2::new(-100f32, -100f32),
            max_linear_speed: Vec2::new(100f32, 100f32),
            min_angular_speed: -10,
            max_angular_speed: 10,
            min_bounds: Vec2::new(-800f32, -400f32),
            max_bounds: Vec2::new(800f32, 400f32),
        }
    }
}