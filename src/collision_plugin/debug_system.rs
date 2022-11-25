pub use bevy::prelude::*;
use crate::aabb::AABB;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::SystemData;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn update_debug_info(
    query: Query<(),(With<PolygonComponent>, With<Transform2d>,  With<AABB>)>,
    mut config: ResMut<CollisionConfig>,
    system_data: ResMut<SystemData>,
)
{
    config.entity_count = query.iter().size_hint().1.unwrap();
    config.broad_time = system_data.broad_time.as_secs_f32() * 1000f32;
    config.narrow_time = system_data.narrow_time.as_secs_f32() * 1000f32;

    config.total_physics_time = config.broad_time + config.narrow_time;

}