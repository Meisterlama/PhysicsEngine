use std::default::Default;
use bevy::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub struct RandomPolyConfig {
    pub min_points: usize,
    pub max_points: usize,
    pub min_radius: i32,
    pub max_radius: i32,
    pub min_rotation: i32,
    pub max_rotation: i32,
    pub min_bounds: Vec2,
    pub max_bounds: Vec2,
}

pub fn create_random_poly(config: &RandomPolyConfig) -> (PolygonComponent, Transform2d, AABB, RigidBody2d)
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
    let transform = Transform2d{
        position,
        rotation
    };
    let aabb = AABB::from(&polygon.get_rotated_points(&transform));

    let rigidbody = RigidBody2d{
        mass: (radius*radius) as f32,
        is_kinematic: false,
    };

    return (polygon, transform, aabb, rigidbody);
}

pub fn create_square(length: f32, height: f32, position: Vec2, rotation: f32) -> (PolygonComponent, Transform2d, AABB, RigidBody2d) {
    let mut points = Vec::with_capacity(4);

    points.push(Vec2::new(-length, -height));
    points.push(Vec2::new(-length, height));
    points.push(Vec2::new(length, height));
    points.push(Vec2::new(length, -height));

    let polygon = PolygonComponent::new(points);
    let transform = Transform2d{
        position,
        rotation,
    };
    let aabb = AABB::from(&polygon.get_rotated_points(&transform));
    let mut rigidbody = RigidBody2d::default();
    rigidbody.is_kinematic = true;

    return (polygon, transform, aabb, rigidbody);
}

impl Default for RandomPolyConfig {
    fn default() -> Self {
        RandomPolyConfig {
            min_points: 3,
            max_points: 12,
            min_radius: 1,
            max_radius: 5,
            min_rotation: 0,
            max_rotation: 360,
            min_bounds: Vec2::new(-800f32, -400f32),
            max_bounds: Vec2::new(800f32, 400f32),
        }
    }
}