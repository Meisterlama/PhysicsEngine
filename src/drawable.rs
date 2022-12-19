use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use crate::transform2d::Transform2d;

pub trait Drawable
{
    fn draw(&self, transform: &Transform2d, lines: &mut ResMut<DebugLines>);
}