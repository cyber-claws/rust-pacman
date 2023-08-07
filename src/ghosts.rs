use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use pathfinding::prelude::bfs;

use crate::constants::*;
use crate::game::*;
use crate::player::PacMan;
use crate::shared::enums::Direction;
use crate::shared::types::Point;
use crate::utils::grid_to_matrix;

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
            direction: Direction::Up,
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
            .insert(TransformBundle::from(Transform::from_xyz(
                BLOCK_SCALE * ghost.1,
                -BLOCK_SCALE * ghost.2,
                0.,
            )))
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(BLOCK_SCALE * 0.75))
                    .with_translation(Vec3::from_array([
                        BLOCK_SCALE * ghost.1,
                        -BLOCK_SCALE * ghost.2,
                        0.1,
                    ])),
                material: materials.add(ColorMaterial::from(get_ghost_color(ghost.0))),
                ..default()
            })
            .insert(Collider::ball((BLOCK_SCALE * 0.045) / 2.))
            .insert(RigidBody::Dynamic)
            .insert(ExternalForce {
                force: Vec2::ZERO,
                torque: 0.,
            })
            .insert(Restitution::coefficient(0.))
            .insert(Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            })
            .insert(KinematicCharacterController {
                offset: CharacterLength::Absolute(0.0),
                ..default()
            })
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
    mut ghost: Query<(&Transform, &mut Ghost)>,
    pacman: Query<&Transform, With<PacMan>>,
    _events: EventReader<CollisionEvent>,
    _game_hud: ResMut<GameState>,
) {
    for (ghost_transform, mut ghost_entity) in ghost.iter_mut() {
        for pacman_transform in pacman.iter() {
            let path = find_path_to(
                Vec2::new(
                    ghost_transform.translation.x.clone(),
                    ghost_transform.translation.y.clone(),
                ),
                Vec2::new(
                    pacman_transform.translation.x,
                    pacman_transform.translation.y,
                ),
            );
            match path {
                Some(way) => {
                    println!(
                        "Must go: {:?}, path length: {}",
                        way.clone(),
                        way.clone().len()
                    );
                    ghost_entity.direction = way[0].direction_from_source.clone();
                }
                None => {
                    println!("No path!!!");
                }
            }
        }
    }
}

pub fn move_ghosts(
    time: Res<Time>,
    mut ghost: Query<(&mut Transform, &mut Ghost, &mut ExternalForce)>,
) {
    for (mut transform, ghost, mut external_force) in &mut ghost.iter_mut() {
        match ghost.direction {
            Direction::Up => {
                external_force.force = Vec2::new(0., MOVE_FORCE * time.delta_seconds());
                transform.translation.y += MOVE_FORCE * time.delta_seconds();
            }
            Direction::Down => {
                external_force.force = Vec2::new(0., -MOVE_FORCE * time.delta_seconds());
                transform.translation.y -= MOVE_FORCE * time.delta_seconds();
            }
            Direction::Left => {
                external_force.force = Vec2::new(-MOVE_FORCE * time.delta_seconds(), 0.);
                transform.translation.x -= MOVE_FORCE * time.delta_seconds();
            }
            Direction::Right => {
                external_force.force = Vec2::new(MOVE_FORCE * time.delta_seconds(), 0.);
                transform.translation.x += MOVE_FORCE * time.delta_seconds();
            }
        }

        // TODO: Should the ghosts teleport?
        if transform.translation.x > (SCREEN_WIDTH + BLOCK_SCALE) {
            transform.translation.x = -BLOCK_SCALE;
        }

        if transform.translation.x < -BLOCK_SCALE {
            transform.translation.x = SCREEN_WIDTH + BLOCK_SCALE;
        }
    }
}

fn find_succesors(point: &Point) -> Vec<Point> {
    let &Point {
        x,
        y,
        direction_from_source: _,
    } = point;
    let mut neighbors = Vec::new();

    for &(dx, dy, direction) in &[
        (-1, 0, Direction::Left),
        (1, 0, Direction::Right),
        (0, -1, Direction::Up),
        (0, 1, Direction::Down),
    ] {
        let new_x = x + dx;
        let new_y = y + dy;

        // Check if the new position is within the grid and is not a wall
        if new_x >= 0
            && new_y >= 0
            && new_x < MAP[0].len() as i32
            && new_y < MAP.len() as i32
            && MAP[new_y as usize][new_x as usize] != '#'
        {
            neighbors.push(Point {
                x: new_x,
                y: new_y,
                direction_from_source: direction,
            });
        }
    }
    neighbors
}

fn find_path_to(start: Vec2, end: Vec2) -> Option<Vec<Point>> {
    let (start_row, start_col) = grid_to_matrix(start);
    let (end_row, end_col) = grid_to_matrix(end);

    let results = bfs(
        &Point {
            x: start_row,
            y: start_col,
            direction_from_source: Direction::Left,
        },
        |p| find_succesors(p),
        |p| {
            *p == Point {
                x: end_row,
                y: end_col,
                direction_from_source: Direction::Left,
            }
        },
    );
    results
}
