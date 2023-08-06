use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::constants::*;

#[derive(Default, Component)]
pub struct Wall;

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..MAP.len() {
        for j in 0..MAP[i].len() {
            let cell = MAP[i][j];
            match cell {
                '#' => {
                    commands
                        .spawn(Wall)
                        .insert(RigidBody::Fixed)
                        .insert(TransformBundle::from(Transform::from_xyz(
                            BLOCK_SCALE * j as f32,
                            -(BLOCK_SCALE * i as f32),
                            0.,
                        )))
                        .insert(Collider::cuboid(
                            BLOCK_SCALE * 0.05 / 2.,
                            BLOCK_SCALE * 0.05 / 2.,
                        ))
                        .insert(Restitution::coefficient(0.))
                        .insert(GravityScale(0.))
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                            transform: Transform::default()
                                .with_scale(Vec3::splat(BLOCK_SCALE))
                                .with_translation(Vec3::from_array([
                                    BLOCK_SCALE * j as f32,
                                    -BLOCK_SCALE * i as f32,
                                    0.,
                                ])),
                            material: materials.add(ColorMaterial::from(Color::BLUE)),
                            ..default()
                        });
                }
                _ => {}
            }
        }
    }
}
