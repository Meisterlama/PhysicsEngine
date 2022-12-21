use bevy::prelude::*;

#[derive(Component)]
pub struct RigidBody2d {
    pub mass: f32,
    pub is_kinematic: bool
}

impl RigidBody2d {
    pub(crate) fn get_mass(&self) -> f32 {
        if self.is_kinematic {
            return f32::MAX
        }
        return self.mass;
    }
}

impl Default for RigidBody2d {
    fn default() -> Self {
        RigidBody2d {
            mass: 10f32,
            is_kinematic: false,
        }
    }
}