use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Transform2d{
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform2d
{
    #[inline]
    pub fn transform_point(&self, mut point: Vec2) -> Vec2 {
        point *= self.scale;
        point = Vec2::from_angle(self.rotation).rotate(point);
        point += self.translation;
        point
    }

    #[inline]
    pub fn inv_transform_point(&self, point: Vec2) -> Vec2 {
        return self.inv_rotate(self.inv_translate(point));
    }

    #[inline]
    pub fn rotate(&self, point: Vec2) -> Vec2 {
        return Vec2::from_angle(self.rotation).rotate(point);
    }

    #[inline]
    pub fn inv_rotate(&self, point: Vec2) -> Vec2 {
        return Vec2::from_angle(-self.rotation).rotate(point);
    }

    #[inline]
    pub fn translate(&self, point: Vec2) -> Vec2 {
        return point + self.translation;
    }

    #[inline]
    pub fn inv_translate(&self, point: Vec2) -> Vec2 {
        return point - self.translation;
    }
}