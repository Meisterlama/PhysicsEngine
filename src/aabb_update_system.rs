use bevy::prelude::*;
use crate::aabb::AABB;
use crate::polygon_component::PolygonComponent;
use crate::polygon_plugin::EntityToMove;
use crate::transform2d::Transform2d;

pub fn aabb_update_system(mut query: Query<(&PolygonComponent, &mut AABB, &Transform2d), (Changed<Transform2d>, Without<EntityToMove>)>)
{
    for (p, mut a, t) in query.iter_mut()
    {
        *a = AABB::from(&p.get_rotated_points(&t));
    }
}