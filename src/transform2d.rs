use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Transform2d{
    pub position: Vec2,
    pub rotation: f32,
}

impl Transform2d
{
    pub fn apply_to(&self, point: &Vec2) -> Vec2 {
        return self.translate(&self.rotate(point));
    }

    pub fn inv_apply_to(&self, point: &Vec2) -> Vec2 {
        return self.inv_rotate(&self.inv_translate(point));
    }


    pub fn rotate(&self, point: &Vec2) -> Vec2 {
        return Vec2::from_angle(self.rotation).rotate(*point);
    }

    pub fn inv_rotate(&self, point: &Vec2) -> Vec2 {
        return Vec2::from_angle(-self.rotation).rotate(*point);
    }

    pub fn translate(&self, point: &Vec2) -> Vec2 {
        return *point + self.position;
    }

    pub fn inv_translate(&self, point: &Vec2) -> Vec2 {
        return *point - self.position;
    }
}