use bevy::prelude::*;

use crate::transform2d::Transform2d;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AABB
{
    pub min: Vec2,
    pub max: Vec2,
    pub draw_color: Color,
}

impl AABB {
    pub fn is_point_inside(&self, point: Vec2, transform: &Transform2d) -> bool {
        let point = point - transform.translation;
        return point.x <= self.max.x &&
            point.x >= self.min.x &&
            point.y <= self.max.y &&
            point.y >= self.min.y;
    }

    pub fn from(points: &Vec<Vec2>) -> Self
    {
        let mut bounding_box = Self {
            min: points[0].clone(),
            max: points[0].clone(),
            ..default()
        };

        for point in points {
            if point.x > bounding_box.max.x {
                bounding_box.max.x = point.x;
            } else if point.x < bounding_box.min.x
            {
                bounding_box.min.x = point.x;
            }

            if point.y > bounding_box.max.y {
                bounding_box.max.y = point.y;
            } else if point.y < bounding_box.min.y {
                bounding_box.min.y = point.y;
            }
        }
        return bounding_box;
    }

    pub fn get_points(&self, t: &Transform2d) -> Vec<Vec3> {
        let min = self.min + t.translation;
        let max = self.max + t.translation;
        return vec!(
            Vec3::new(min.x, min.y, 0f32),
            Vec3::new(max.x, min.y, 0f32),
            Vec3::new(max.x, max.y, 0f32),
            Vec3::new(min.x, max.y, 0f32),
            Vec3::new(min.x, min.y, 0f32),
        );
    }
}

impl Default for AABB {
    fn default() -> Self {
        AABB {
            min: Vec2::default(),
            max: Vec2::default(),
            draw_color: Color::GRAY,
        }
    }
}

pub fn check_collision(a1: &AABB, t1: &Transform2d, a2: &AABB, t2: &Transform2d) -> bool
{
    let a1_min = a1.min + t1.translation;
    let a1_max = a1.max + t1.translation;
    let a2_min = a2.min + t2.translation;
    let a2_max = a2.max + t2.translation;
    return a1_min.x <= a2_max.x &&
        a1_max.x >= a2_min.x &&
        a1_min.y <= a2_max.y &&
        a1_max.y >= a2_min.y;
}
