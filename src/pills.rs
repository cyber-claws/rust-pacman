use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::constants::*;
use crate::game::*;
use crate::player::*;

#[derive(Default, Component)]
pub struct Pill;

#[derive(Default, Component)]
pub struct PowerPill;

pub fn setup_pills(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..MAP.len() {
        for j in 0..MAP[i].len() {
            let cell = MAP[i][j];
            match cell {
                '*' => {
                    commands
                        .spawn(Pill)
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                            transform: Transform::default()
                                .with_scale(Vec3::splat(BLOCK_SCALE * 0.25))
                                .with_translation(Vec3::from_array([
                                    BLOCK_SCALE * j as f32,
                                    -BLOCK_SCALE * i as f32,
                                    0.,
                                ])),
                            material: materials.add(ColorMaterial::from(Color::WHITE)),
                            ..default()
                        })
                        .insert(RigidBody::Fixed)
                        .insert(Collider::ball(BLOCK_SCALE * 0.05 / 2.))
                        .insert(Sensor)
                        .insert(Restitution::coefficient(0.))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(GravityScale(0.));
                }
                'P' => {
                    commands
                        .spawn(PowerPill)
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                            transform: Transform::default()
                                .with_scale(Vec3::splat(BLOCK_SCALE * 0.5))
                                .with_translation(Vec3::from_array([
                                    BLOCK_SCALE * j as f32,
                                    -BLOCK_SCALE * i as f32,
                                    0.,
                                ])),
                            material: materials.add(ColorMaterial::from(Color::WHITE)),
                            ..default()
                        })
                        .insert(RigidBody::Fixed)
                        .insert(Collider::ball(BLOCK_SCALE * 0.05 / 2.))
                        .insert(Sensor)
                        .insert(Restitution::coefficient(0.))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(GravityScale(0.));
                }
                _ => {}
            }
        }
    }
}

pub fn normal_pills_update(
    mut commands: Commands,
    mut pill: Query<Entity, With<Pill>>,
    pacman: Query<Entity, With<PacMan>>,
    mut events: EventReader<CollisionEvent>,
    mut game_hud: ResMut<GameState>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if let (Ok(small_pill), Ok(_pac_man)) = (pill.get_mut(*a), pacman.get(*b)) {
                    commands.entity(small_pill).despawn_recursive();
                    game_hud.score += NORMAL_PILL_POINTS;
                } else if let (Ok(entity), Ok(_pac_man)) = (pill.get_mut(*b), pacman.get(*a)) {
                    commands.entity(entity).despawn_recursive();
                    game_hud.score += NORMAL_PILL_POINTS;
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
    events.clear()
}

pub fn power_pills_update(
    mut commands: Commands,
    mut pill: Query<Entity, With<PowerPill>>,
    pacman: Query<Entity, With<PacMan>>,
    mut events: EventReader<CollisionEvent>,
    mut game_hud: ResMut<GameState>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if let (Ok(power_pill), Ok(_pac_man)) = (pill.get_mut(*a), pacman.get(*b)) {
                    commands.entity(power_pill).despawn_recursive();
                    game_hud.pill_cool_down = Timer::from_seconds(0., TimerMode::Once);
                    game_hud.score += POWER_PILL_POINTS;
                } else if let (Ok(entity), Ok(_pac_man)) = (pill.get_mut(*b), pacman.get(*a)) {
                    commands.entity(entity).despawn_recursive();
                    game_hud.pill_cool_down = Timer::from_seconds(0., TimerMode::Once);
                    game_hud.score += POWER_PILL_POINTS;
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
    events.clear()
}
