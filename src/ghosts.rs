use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::constants::*;
use crate::shared::enums::Direction;

#[derive(Clone, Copy)]
pub enum GhostNames {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

impl Default for GhostNames {
    fn default() -> Self {
        GhostNames::Pinky
    }
}

#[derive(Default, Component)]
pub struct Ghost {
    pub name: GhostNames,
    pub direction: Direction,
}

impl Ghost {
    pub fn new(name: GhostNames) -> Ghost {
        Ghost {
            name,
            ..Default::default()
        }
    }
}

pub fn setup_ghosts(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ghost in GHOST_INITIAL_POSITION.iter() {
        commands
            .spawn(Ghost::new(ghost.0))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(BLOCK_SCALE * 0.4))
                    .with_translation(Vec3::from_array([
                        SCREEN_BOTTOM_X + (BLOCK_SCALE * ghost.1),
                        SCREEN_BOTTOM_Y - (BLOCK_SCALE * ghost.2),
                        0.5, // Add a little on the z-index just to make sure the ghosts are always about the pills
                    ])),
                material: materials.add(ColorMaterial::from(get_ghost_color(ghost.0))),
                ..default()
            });
    }
}

fn get_ghost_color(ghost: GhostNames) -> Color {
    match ghost {
        GhostNames::Inky => Color::CYAN,
        GhostNames::Blinky => Color::RED,
        GhostNames::Clyde => Color::ORANGE,
        GhostNames::Pinky => Color::PINK,
    }
}

pub fn _ghosts_tick(
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
}
