pub use bevy::prelude::*;
use crate::aabb::AABB;
use crate::collision_plugin::config::CollisionConfig;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

#[allow(dead_code)]
pub fn update_debug_info(
    query: Query<(),(With<PolygonComponent>, With<Transform2d>,  With<AABB>)>,
    mut config: ResMut<CollisionConfig>,
)
{
    config.entity_count = query.iter().size_hint().1.unwrap();
}