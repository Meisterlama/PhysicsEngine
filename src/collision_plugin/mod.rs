use bevy::prelude::*;
#[allow(unused_imports)]
use bevy_inspector_egui::Inspectable;
#[allow(unused_imports)]
use bevy_inspector_egui::InspectorPlugin;

mod data_structs;
mod config;
pub mod rigidbody;
mod systems;
pub mod plugin;
pub mod rendering;
pub mod aabb;
pub mod polygon_component;
pub mod polygon_plugin;
mod broad_phase;
mod narrow_phase;
mod helpers;
mod collision_response;
mod debug;

#[derive(Default, Component)]
pub struct PhysicsAwake;

//FIXME: TIMESTEP is not implemented as it currently breaks ordering
const TIMESTEP: f64 = 1.0 /60.0;

