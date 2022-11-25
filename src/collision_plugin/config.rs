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
}

impl Default for CollisionConfig
{
    fn default() -> Self {
        CollisionConfig {
            broad_phase_type: BroadPhaseType::SAP,
            narrow_phase_type: NarrowPhaseType::Enabled,
            draw_debug_broad_phase: false,
            draw_debug_narrow_phase: false,
            draw_debug_aabb: false,
            entity_count: 0usize,
            broad_time: 0f32,
            narrow_time: 0f32,
            total_physics_time: 0f32,
        }
    }
}