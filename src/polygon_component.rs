use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::drawable::Drawable;
use crate::transform2d::Transform2d;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PolygonComponent
{
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl PolygonComponent {
    pub fn new(points: Vec<Vec2>) -> Self
    {
        PolygonComponent {
            points,
            color: Color::GREEN,
        }
    }
    pub fn get_transformed_points(&self, transform: &Transform2d) -> Vec<Vec2> {
        return self.points.iter().map(|p| transform.apply_to(p)).collect();
    }

    pub fn get_translated_points(&self, transform: &Transform2d) -> Vec<Vec2> {
        return self.points.iter().map(|p| transform.translate(p)).collect();
    }

    pub fn get_rotated_points(&self, transform: &Transform2d) -> Vec<Vec2> {
        return self.points.iter().map(|p| transform.rotate(p)).collect();
    }
}

impl Drawable for PolygonComponent {
    fn draw(&self, transform: &Transform2d, lines: &mut ResMut<DebugLines>) {
        let points = self.get_transformed_points(transform);
        for pair in points.windows(2) {
            lines.line_colored(pair[0].extend(0f32),
                               pair[1].extend(0f32),
                               0.0,
                               self.color);
        }
        lines.line_colored(points.first().unwrap().extend(0f32), points.last().unwrap().extend(0f32), 0f32, self.color)
    }
}