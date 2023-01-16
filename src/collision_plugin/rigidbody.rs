use bevy::prelude::*;

#[derive(Component)]
pub struct RigidBody2d {
    pub mass: f32,
    pub friction: f32,
    pub is_kinematic: bool,

    pub linear_speed: Vec2,
    pub angular_speed: f32,

    pub linear_acceleration: Vec2,
    pub angular_acceleration: f32,
}

impl RigidBody2d {
    pub(crate) fn get_mass(&self) -> f32 {
        if self.is_kinematic {
            return f32::MAX;
        }
        return self.mass;
    }

    pub(crate) fn get_inv_mass(&self) -> f32 {
        if self.is_kinematic {
            return  0f32;
        }
        return 1f32/self.mass
    }
}

impl Default for RigidBody2d {
    fn default() -> Self {
        RigidBody2d {
            mass: 10f32,
            friction: 0.9f32,
            is_kinematic: false,
            linear_speed: Vec2::ZERO,
            angular_speed: 0f32,
            // linear_acceleration: Vec2::new(0f32, -9.8f32),
            linear_acceleration: Vec2::ZERO,
            angular_acceleration: 0f32
        }
    }
}