use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

/**
 * '#' -> Wall
 * '*' -> Normal Pill
 * 'P' -> Power Pill
 * 'B' -> Blinky
 * 'Q' -> Pinky
 * 'I' -> Inky
 * 'C' -> Clyde
 * 'H' -> Pac-Man
 * 'F' -> Fruit Spot
 * '0' -> Empty Space
 */
const MAP: [[char; 19]; 24] = [
    ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
    ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
	['#', '*', '*', '*', '*', '*', '*', '*', '*', '#', '*', '*', '*', '*', '*', '*', '*', '*', '#'],
	['#', 'P', '#', '#', '*', '#', '#', '#', '*', '#', '*', '#', '#', '#', '*', '#', '#', 'P', '#'],
	['#', '*', '#', '#', '*', '#', '#', '#', '*', '#', '*', '#', '#', '#', '*', '#', '#', '*', '#'],
	['#', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '#'],
	['#', '*', '#', '#', '*', '#', '*', '#', '#', '#', '#', '#', '*', '#', '*', '#', '#', '*', '#'],
	['#', '*', '*', '*', '*', '#', '*', '*', '*', '#', '*', '*', '*', '#', '*', '*', '*', '*', '#'],
	['#', '#', '#', '#', '*', '#', '#', '#', '*', '#', '*', '#', '#', '#', '*', '#', '#', '#', '#'],
	['0', '0', '0', '#', '*', '#', '*', '*', '*', '*', '*', '*', '*', '#', '*', '#', '0', '0', '0'],
	['#', '#', '#', '#', '*', '#', '*', '#', '#', 'B', '#', '#', '*', '#', '*', '#', '#', '#', '#'],
	['0', '0', '0', '0', '*', '*', '*', '#', 'Q', 'I', 'C', '#', '*', '*', '*', '0', '0', '0', '0'],
	['#', '#', '#', '#', '*', '#', '*', '#', '#', '#', '#', '#', '*', '#', '*', '#', '#', '#', '#'],
	['0', '0', '0', '#', '*', '#', '*', '*', '*', 'F', '*', '*', '*', '#', '*', '#', '0', '0', '0'],
	['#', '#', '#', '#', '*', '#', '*', '#', '#', '#', '#', '#', '*', '#', '*', '#', '#', '#', '#'],
	['#', '*', '*', '*', '*', '*', '*', '*', '*', '#', '*', '*', '*', '*', '*', '*', '*', '*', '#'],
	['#', '*', '#', '#', '*', '#', '#', '#', '*', '#', '*', '#', '#', '#', '*', '#', '#', '*', '#'],
	['#', 'P', '*', '#', '*', '*', '*', '*', '*', 'H', '*', '*', '*', '*', '*', '#', '*', 'P', '#'],
	['#', '#', '*', '#', '*', '#', '*', '#', '#', '#', '#', '#', '*', '#', '*', '#', '*', '#', '#'],
	['#', '*', '*', '*', '*', '#', '*', '*', '*', '#', '*', '*', '*', '#', '*', '*', '*', '*', '#'],
	['#', '*', '#', '#', '#', '#', '#', '#', '*', '#', '*', '#', '#', '#', '#', '#', '#', '*', '#'],
	['#', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '*', '#'],
	['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0']
];

const MOVE_FORCE: f32 =  100.;
const BLOCK_SCALE: f32 = 24.;
const SCREEN_WIDTH: f32 = 456.;
const SCREEN_HEIGHT: f32 = 576.;
const SCREEN_BOTTOM_Y: f32 = SCREEN_HEIGHT / 2. - 12.; // 12.0 is half of the block, helps center the grid.
const SCREEN_BOTTOM_X: f32 = -SCREEN_WIDTH / 2. + 12.; // 12.0 is half of the block, helps center the grid.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                title: "Pac-Man".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup_camera, setup_sounds, draw_map))
        .add_systems(Update, (update_pacman, update_normal_pills))
        .run();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}

#[derive(Default, Component)]
struct Pill;

#[derive(Default, Component)]
struct PacMan {
    direction: Direction,
}

impl PacMan {
    pub fn new() -> PacMan {
        PacMan {
            ..Default::default()
        }
    }
}

#[derive(Resource)]
struct PacManChompSound(Handle<AudioSource>);

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

fn setup_camera(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Pac-Man chomp sound
    let pacman_chomp_sound = asset_server.load("sounds/pacman_chomp.ogg");
    commands.insert_resource(PacManChompSound(pacman_chomp_sound));
}

fn draw_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..MAP.len() {
        for j in 0..MAP[i].len() {
            let cell = MAP[i][j];
            match cell {
                '#' => {
                    commands.spawn(RigidBody::Fixed)
                        .insert(TransformBundle::from(Transform::from_xyz(SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.)))
                        .insert(Collider::cuboid(BLOCK_SCALE * 0.05 / 2., BLOCK_SCALE * 0.05 / 2.))
                        .insert(Restitution::coefficient(0.))
                        .insert(GravityScale(0.))
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                            transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                            material: materials.add(ColorMaterial::from(Color::BLUE)),
                            ..default()
                        });
                },
                'H' => {
                    commands.spawn(PacMan::new())
                        .insert(RigidBody::Dynamic)
                        .insert(TransformBundle::from(Transform::from_xyz(SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.)))
                        .insert(Collider::ball((BLOCK_SCALE * 0.05) / 2.))
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
                            transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.75)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                            material: materials.add(ColorMaterial::from(Color::YELLOW)),
                            ..default()
                        });
                },
                '*' => {
                    commands
                        .spawn(Pill)
                        .insert(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                            transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.25)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                            material: materials.add(ColorMaterial::from(Color::WHITE)),
                            ..default()
                        })
                        .insert(RigidBody::Fixed)
                        .insert(Collider::ball(BLOCK_SCALE * 0.05 / 2.))
                        .insert(Sensor)
                        .insert(Restitution::coefficient(0.))
                        .insert(ActiveEvents::COLLISION_EVENTS)
                        .insert(GravityScale(0.));
                },
                'P' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.5)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        ..default()
                    });
                },
                'I' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.5)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                        ..default()
                    });
                },
                'Q' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.5)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                        material: materials.add(ColorMaterial::from(Color::PINK)),
                        ..default()
                    });
                },
                'B' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.5)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                        material: materials.add(ColorMaterial::from(Color::RED)),
                        ..default()
                    });
                },
                'C' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(BLOCK_SCALE * 0.5)).with_translation(Vec3::from_array([SCREEN_BOTTOM_X + (BLOCK_SCALE * j as f32), SCREEN_BOTTOM_Y - (BLOCK_SCALE * i as f32), 0.])),
                        material: materials.add(ColorMaterial::from(Color::ORANGE)),
                        ..default()
                    });
                },
                _ => {}
            }   
        }
    }
}

fn update_pacman(
    mut characters: Query<(&mut Transform, &mut PacMan, &mut ExternalForce)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut character, mut external_force) in &mut characters.iter_mut() {
        if input.pressed(KeyCode::Up) {
            character.direction = Direction::Up
        }
        if input.pressed(KeyCode::Down) {
            character.direction = Direction::Down
        }
        if input.pressed(KeyCode::Left) {
            character.direction = Direction::Left
        }
        if input.pressed(KeyCode::Right) {
            character.direction = Direction::Right
        }

        match character.direction {
            Direction::Up => {
                external_force.force = Vec2::new(external_force.force.x, MOVE_FORCE * time.delta_seconds());
                transform.translation.y += MOVE_FORCE * time.delta_seconds();
            }
            Direction::Down => {
                external_force.force = Vec2::new(external_force.force.x, -MOVE_FORCE * time.delta_seconds());
                transform.translation.y -= MOVE_FORCE * time.delta_seconds();
            }
            Direction::Left => {
                external_force.force = Vec2::new(-MOVE_FORCE * time.delta_seconds(), external_force.force.y);
                transform.translation.x -= MOVE_FORCE * time.delta_seconds();
            }
            Direction::Right => {
                external_force.force = Vec2::new(MOVE_FORCE * time.delta_seconds(), external_force.force.y);
                transform.translation.x += MOVE_FORCE * time.delta_seconds();
            }
        }

        // Handle portaling
        if transform.translation.x > ((SCREEN_WIDTH / 2.) + BLOCK_SCALE + 0.5) {
            transform.translation.x = -((SCREEN_WIDTH / 2.) + BLOCK_SCALE + 0.5);
        }

        if transform.translation.x < -((SCREEN_WIDTH / 2.) + BLOCK_SCALE + 0.5) {
            transform.translation.x = ((SCREEN_WIDTH / 2.) + BLOCK_SCALE + 0.5);
        }
    }
}

fn update_normal_pills(
    mut commands: Commands,
    mut pill: Query<Entity, With<Pill>>,
    pacman: Query<Entity, With<PacMan>>,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                if let (Ok(small_pill), Ok(pac_man)) =
                    (pill.get_mut(*a), pacman.get(*b))
                {
                    // Despawn on colision
                    commands
                        .entity(small_pill)
                        .despawn_recursive();             
                } else if let (Ok(entity), Ok(pac_man)) =
                    (pill.get_mut(*b), pacman.get(*a))
                {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
    events.clear()
}

fn play_chomp_sound(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    chomp_sound: Res<PacManChompSound>,
) {
    if !collision_events.is_empty() {
        commands.spawn(AudioBundle {
            source: chomp_sound.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}