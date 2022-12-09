use std::time::Duration;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable)]
pub enum BroadPhaseType {
    Disabled,
    Rough,
    SAP,
}

#[derive(Inspectable)]
pub enum NarrowPhaseType {
    Disabled,
    Enabled,
}

#[derive(Resource, Inspectable)]
pub struct CollisionConfig {
    pub broad_phase_type: BroadPhaseType,
    pub narrow_phase_type: NarrowPhaseType,
    #[inspectable(label = "Put non colliding objects to sleep (Bugged: Wake up doesn't work correctly)")]
    pub put_non_colliding_asleep: bool,

    pub draw_debug_broad_phase: bool,
    pub draw_debug_narrow_phase: bool,
    pub draw_debug_aabb: bool,
    #[inspectable(read_only)]
    pub entity_count: usize,
    #[inspectable(read_only, label = "Broad phase time", suffix = " ms")]
    pub broad_time: f32,
    #[inspectable(read_only, label = "Narrow phase time", suffix = " ms")]
    pub narrow_time: f32,
    #[inspectable(read_only, label = "Total physic time", suffix = " ms")]
    pub total_physics_time: f32,
    #[inspectable(read_only, label = "Total frame time", suffix = " ms")]
    pub total_frame_time: f32,

    #[inspectable(read_only)]
    pub collision_pairs_count: usize,
    #[inspectable(read_only)]
    pub awake_entities_count: usize,
}

impl Default for CollisionConfig
{
    fn default() -> Self {
        CollisionConfig {
            broad_phase_type: BroadPhaseType::SAP,
            narrow_phase_type: NarrowPhaseType::Enabled,
            put_non_colliding_asleep: false,
            draw_debug_broad_phase: false,
            draw_debug_narrow_phase: false,
            draw_debug_aabb: false,
            entity_count: 0usize,
            broad_time: 0f32,
            narrow_time: 0f32,
            total_physics_time: 0f32,
            total_frame_time: 0f32,
            collision_pairs_count: 0usize,
            awake_entities_count: 0usize,
        }
    }
}