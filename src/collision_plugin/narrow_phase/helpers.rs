use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn get_projection(polygon: &PolygonComponent, transform: &Transform2d, axis: Vec2) -> Vec2
{
    let min = polygon.get_transformed_points(transform).iter()
        .map(move |&p| p.dot(axis))
        .min_by(move |lhs, rhs| lhs.total_cmp(rhs))
        .unwrap();

    let max = polygon.get_transformed_points(transform).iter()
        .map(move |&p| p.dot(axis))
        .max_by(move |lhs, rhs| lhs.total_cmp(rhs))
        .unwrap();

    return Vec2::new(min, max);
}

pub fn overlaps(lhs: Vec2, rhs: Vec2) -> bool
{
    return lhs.y > rhs.x && lhs.x < rhs.y;
}

pub fn triple_product(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec2
{
    let a = a.extend(0f32);
    let b = b.extend(0f32);
    let c = c.extend(0f32);

    let fc = a.cross(b);
    let sc = fc.cross(c);

    return sc.xy();
}

pub fn find_furthest_point(polygon: &PolygonComponent, transform: &Transform2d, direction: Vec2) -> Vec2
{
    return polygon.get_transformed_points(transform).iter()
        .max_by(move |lhs, rhs| {
            let lhs_dot = lhs.dot(direction);
            let rhs_dot = rhs.dot(direction);
            return lhs_dot.total_cmp(&rhs_dot);
        })
        .unwrap().clone();
}