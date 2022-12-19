pub use bevy::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::broad_phase::BroadPhaseData;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::narrow_phase::NarrowPhaseData;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub fn update_debug_info(
    query: Query<(),(With<PolygonComponent>, With<Transform2d>,  With<AABB>)>,
    mut config: ResMut<CollisionConfig>,
    time: Res<Time>,
    broad_phase_data: ResMut<BroadPhaseData>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
)
{
    config.entity_count = query.iter().size_hint().1.unwrap();
    config.broad_time = broad_phase_data.time.as_secs_f32() * 1000f32;
    config.narrow_time = narrow_phase_data.time.as_secs_f32() * 1000f32;

    config.total_physics_time = config.broad_time + config.narrow_time;
    config.total_frame_time = time.delta_seconds() * 1000f32;

    config.collision_pairs_count = broad_phase_data.collision_pairs.len();
    config.awake_entities_count = broad_phase_data.sorted_entities.len();
}