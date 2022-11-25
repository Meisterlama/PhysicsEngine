use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::drawable::Drawable;
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
    pub fn is_point_inside(&self, point: Vec2) -> bool {
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

impl Drawable for AABB {
    fn draw(&self, transform: &Transform2d, lines: &mut ResMut<DebugLines>) {
        lines.line_colored(
            transform.translate(&Vec2::new(self.min.x, self.min.y)).extend(0f32),
            transform.translate(&Vec2::new(self.max.x, self.min.y)).extend(0f32),
            0f32,
            self.draw_color,
        );
        lines.line_colored(
            transform.translate(&Vec2::new(self.max.x, self.min.y)).extend(0f32),
            transform.translate(&Vec2::new(self.max.x, self.max.y)).extend(0f32),
            0f32,
            self.draw_color,
        );
        lines.line_colored(
            transform.translate(&Vec2::new(self.max.x, self.max.y)).extend(0f32),
            transform.translate(&Vec2::new(self.min.x, self.max.y)).extend(0f32),
            0f32,
            self.draw_color,
        );
        lines.line_colored(
            transform.translate(&Vec2::new(self.min.x, self.max.y)).extend(0f32),
            transform.translate(&Vec2::new(self.min.x, self.min.y)).extend(0f32),
            0f32,
            self.draw_color,
        );
    }
}

pub fn check_collision(a1: &AABB, t1: &Transform2d, a2: &AABB, t2: &Transform2d) -> bool
{
    let a1_min = t1.translate(&a1.min);
    let a1_max = t1.translate(&a1.max);
    let a2_min = t2.translate(&a2.min);
    let a2_max = t2.translate(&a2.max);
    return a1_min.x <= a2_max.x &&
        a1_max.x >= a2_min.x &&
        a1_min.y <= a2_max.y &&
        a1_max.y >= a2_min.y;
}

