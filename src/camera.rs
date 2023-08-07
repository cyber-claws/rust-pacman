use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::constants::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            // Move camera so that (0, 0) is at the top left-most corner.
            transform: Transform::default().with_translation(Vec3::from_array([
                (SCREEN_WIDTH / 2.) - (BLOCK_SCALE * 0.5),
                (-SCREEN_HEIGHT / 2.) + (BLOCK_SCALE * 0.5),
                1.,
            ])),
            ..default()
        },
        BloomSettings {
            intensity: 0.2,
            ..default()
        },
    ));
}
