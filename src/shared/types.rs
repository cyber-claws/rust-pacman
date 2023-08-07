use bevy::prelude::*;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn to_display_coordinate(&self) -> Vec2<f32> {
        Vec2::new()
    }
}
