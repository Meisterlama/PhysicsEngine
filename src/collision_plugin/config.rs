use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Default)]
pub enum BroadPhaseType {
    Disabled,
    Rough,
    #[default]
    SAP,
}

#[derive(Inspectable, Default)]
pub enum NarrowPhaseType {
    Disabled,
    SAT,
    #[default]
    GJK,
}

#[derive(Inspectable)]
pub struct PhaseConfig {
    pub broad_phase_type: BroadPhaseType,
    pub narrow_phase_type: NarrowPhaseType,

    pub compute_info_collision: bool,

    pub multithread_broad_phase: bool,
    pub multithread_narrow_phase: bool,
    #[inspectable(label = "multithread_response_phase\n(Bugged: collisions explode due tu lack of clamping)")]

    pub multithread_response_phase: bool,
}

impl Default for PhaseConfig {
    fn default() -> Self {
        PhaseConfig{
            broad_phase_type: Default::default(),
            narrow_phase_type: Default::default(),
            compute_info_collision: true,
            multithread_broad_phase: true,
            multithread_narrow_phase: true,
            multithread_response_phase: false,
        }
    }
}

#[derive(Inspectable, Default)]
pub struct DebugDrawing {
    pub draw_debug_broad_phase: bool,
    pub draw_debug_narrow_phase: bool,
    pub draw_debug_aabb: bool,
    pub draw_debug_rigidbody: bool,
}

#[derive(Inspectable)]
pub struct SystemParams {
    #[inspectable(min = 0.01, max = 1.0)]
    pub damping: f32,
    #[inspectable(min = 0.01, max = 1.0)]
    pub restitution: f32,

    #[inspectable(min = 0.01, max = 1.0)]
    pub friction: f32,

    pub gravity_enabled: bool,
}

impl Default for SystemParams {
    fn default() -> Self {
        Self {
            damping: 0.2f32,
            restitution: 0.6f32,
            friction: 0.2f32,
            gravity_enabled: false,
        }
    }
}

#[derive(Inspectable, Default)]
pub struct Statistics {
    #[inspectable(read_only)]
    pub entity_count: usize,
    #[inspectable(read_only, label = "Broad phase time", suffix = " ms")]
    pub broad_time: f32,
    #[inspectable(read_only, label = "Narrow phase time", suffix = " ms")]
    pub narrow_time: f32,
    #[inspectable(read_only, label = "Collision Response handling time", suffix = " ms")]
    pub collision_response_time: f32,
    #[inspectable(read_only, label = "Total physic time", suffix = " ms")]
    pub total_physics_time: f32,
    #[inspectable(read_only, label = "Total frame time", suffix = " ms")]
    pub total_frame_time: f32,

    #[inspectable(read_only)]
    pub collision_pairs_count: usize,
    #[inspectable(read_only)]
    pub awake_entities_count: usize,
}

#[derive(Resource, Inspectable)]
pub struct CollisionConfig {

    #[inspectable(collapse)]
    pub phase_config: PhaseConfig,

    #[inspectable(collapse)]
    pub debug_drawing: DebugDrawing,

    #[inspectable(collapse)]
    pub statistics: Statistics,

    #[inspectable(collapse)]
    pub system_params: SystemParams,

    // #[inspectable(label = "Put non colliding objects to sleep\n(Bugged: Wake up doesn't work correctly)")]
    // pub put_non_colliding_asleep: bool,
}

impl Default for CollisionConfig
{
    fn default() -> Self {
        CollisionConfig {
            phase_config: PhaseConfig::default(),
            debug_drawing: DebugDrawing::default(),
            statistics: Statistics::default(),
            system_params: SystemParams::default(),
        }
    }
}