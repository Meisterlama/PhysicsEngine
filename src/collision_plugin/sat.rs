use bevy::prelude::*;

use crate::collision_plugin::collision_structs::CollisionInfo;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn get_sat_axes_to_test(polygon: &PolygonComponent, transform: &Transform2d) -> Vec<Vec2>
{
    let mut axes = Vec::with_capacity(polygon.points.len());

    for p in polygon.get_transformed_points(transform).windows(2) {
        let edge = p[0] - p[1];
        axes.push(edge.perp());
    }

    return axes;
}

fn get_projection(polygon: &PolygonComponent, transform: &Transform2d, axis: Vec2) -> Vec2
{
    let min = polygon.get_transformed_points(transform).iter().map(move |p| axis.dot(*p)).min_by(move |lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();
    let max = polygon.get_transformed_points(transform).iter().map(move |p| axis.dot(*p)).max_by(move |lhs, rhs| lhs.partial_cmp(rhs).unwrap()).unwrap();

    return Vec2::new(min, max);
}

fn overlaps(lhs: Vec2, rhs: Vec2) -> bool
{
    return lhs.y > rhs.x && lhs.x < rhs.y;
}

pub fn check_collision(p1: &PolygonComponent, t1: &Transform2d,
                       p2: &PolygonComponent, t2: &Transform2d) -> (bool, CollisionInfo) {
    let axes_p1 = get_sat_axes_to_test(p1, t1);
    for axis in axes_p1 {
        let p1 = get_projection(p1, t1, axis);
        let p2 = get_projection(p2, t2, axis);

        if !overlaps(p1, p2)
        {
            return (false, CollisionInfo::default());
        }
    }

    let axes_poly2 = get_sat_axes_to_test(p2, t2);
    for axis in axes_poly2 {
        let p1 = get_projection(p1, t1, axis);
        let p2 = get_projection(p2, t2, axis);

        if !overlaps(p1, p2)
        {
            return (false, CollisionInfo::default());
        }
    }

    return (true, CollisionInfo::default())
}