

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::constants::*;
use crate::game::*;
use crate::player::PacMan;
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
                        BLOCK_SCALE * ghost.1,
                        -BLOCK_SCALE * ghost.2,
                        0.,
                    ])),
                material: materials.add(ColorMaterial::from(get_ghost_color(ghost.0))),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(BLOCK_SCALE * 0.06 / 2.))
            .insert(Sensor)
            .insert(Restitution::coefficient(0.))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(GravityScale(0.));
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

pub fn ghosts_update(
    _commands: Commands,
    mut ghost: Query<Entity, With<Ghost>>,
    pacman: Query<Entity, With<PacMan>>,
    mut events: EventReader<CollisionEvent>,
    _game_hud: ResMut<GameState>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if let (Ok(_ghost), Ok(_pac_man)) = (ghost.get_mut(*a), pacman.get(*b)) {
                    // Maybe hit by Ghost
                    panic!("Oops, you just got hit by a ghost");
                } else if let (Ok(_ghost), Ok(_pac_man)) = (ghost.get_mut(*b), pacman.get(*a)) {
                    // Maybe hit by Ghost
                    panic!("Oops, you just got hit by a ghost");
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
    events.clear()
}
