use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::constants::*;

use crate::shared::enums::Direction;

#[derive(Default, Component)]
pub struct PacMan {
    direction: Direction,
}

impl PacMan {
    pub fn new() -> PacMan {
        PacMan {
            ..Default::default()
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let texture_handle = asset_server.load("images/sprite.png");
    // let texture_atlas =
    //     TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 10, 14, None, None);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(PacMan::new())
        // .insert((
        //     SpriteSheetBundle {
        //         texture_atlas: texture_atlas_handle,
        //         sprite: TextureAtlasSprite::new(animation_indices.first),
        //         // transform: Transform::from_scale(Vec3::splat(6.0)),
        //         ..default()
        //     },
        //     animation_indices,
        //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        // ))
        .insert(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(
            BLOCK_SCALE * PACKMAN_INITIAL_POSITION.0,
            -BLOCK_SCALE * PACKMAN_INITIAL_POSITION.1,
            0.,
        )))
        .insert(Collider::ball((BLOCK_SCALE * 0.045) / 2.))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        })
        .insert(GravityScale(0.))
        .insert(Restitution::coefficient(1.))
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(0.0),
            ..default()
        })
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(BLOCK_SCALE * 0.75))
                .with_translation(Vec3::from_array([
                    BLOCK_SCALE * PACKMAN_INITIAL_POSITION.0,
                    -BLOCK_SCALE * PACKMAN_INITIAL_POSITION.1,
                    0.,
                ])),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            ..default()
        });
}

pub fn player_update(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut characters: Query<(&mut Transform, &mut PacMan, &mut ExternalForce)>,
) {
    for (mut transform, mut character, mut external_force) in &mut characters.iter_mut() {
        if input.pressed(KeyCode::Up) {
            character.direction = Direction::Up;
        }
        if input.pressed(KeyCode::Down) {
            character.direction = Direction::Down;
        }
        if input.pressed(KeyCode::Left) {
            character.direction = Direction::Left;
        }
        if input.pressed(KeyCode::Right) {
            character.direction = Direction::Right;
        }

        match character.direction {
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

        // Handle portaling
        if transform.translation.x > (SCREEN_WIDTH + BLOCK_SCALE) {
            transform.translation.x = -BLOCK_SCALE;
        }

        if transform.translation.x < -BLOCK_SCALE {
            transform.translation.x = SCREEN_WIDTH + BLOCK_SCALE;
        }
        // println!(
        //     "Pac-Man is at: ({}, {})",
        //     (transform.translation.x.clone().abs() / BLOCK_SCALE).ceil() as u32,
        //     (transform.translation.y.clone().abs() / BLOCK_SCALE).ceil() as u32
        // );
    }
}
