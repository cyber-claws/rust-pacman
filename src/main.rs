use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;

const MAP: [[char; 28]; 20] = [
    ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', 'E', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ['#', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '#', 'E', '#', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '#'],
    ['#', 'E', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', 'E', '#', 'E', '#', 'E', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', 'E', '#', 'E', '#', 'E', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', 'E', '#', 'E', '#', 'E', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', 'E', 'E', 'E', 'E', '#', '#', '#', '#', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '#', '#', '#', '#', 'E', 'E', 'E', 'E', 'E', 'E', '#'],
    ['#', '#', '#', '#', '#', 'E', '#', '#', ' ', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#', '#', 'E', '#', '#', '#', '#', '#', '#'],
    [' ', ' ', ' ', ' ', '#', 'E', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', 'E', '#', ' ', ' ', ' ', ' ', ' '],
    ['#', '#', '#', '#', '#', 'E', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', 'E', '#', '#', '#', '#', '#', '#'],
    [' ', ' ', ' ', ' ', ' ', 'E', 'E', 'E', ' ', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', ' ', 'E', 'E', 'E', ' ', ' ', ' ', ' ', ' ', ' '],
    ['#', '#', '#', '#', '#', 'E', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', 'E', '#', '#', '#', '#', '#', '#'],
    [' ', ' ', ' ', ' ', '#', 'E', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', 'E', '#', ' ', ' ', ' ', ' ', ' '],
    ['#', '#', '#', '#', '#', 'E', '#', '#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#', '#', ' ', '#', '#', 'E', '#', '#', '#', '#', '#', '#'],
    ['#', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '#'],
    ['#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', 'E', '#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', 'E', '#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', 'E', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', 'E', '#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', 'E', 'E', '#', '#'],
    ['#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', 'E', '#', 'E', '#', '#', '#', '#', 'E', '#', '#', '#', '#', 'E', '#', '#'],
    ['#', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', 'E', '#'],
    ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
];


fn main() {    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                title: "Pac-Man".into(),
                resolution: (480., 672.).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, draw_map)
        .run();
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
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
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(24.)).with_translation(Vec3::from_array([25.0 * i as f32, 25.0 * j as f32, 0.0])),
                        material: materials.add(ColorMaterial::from(Color::BLUE)),
                        ..default()
                    });
                },
                ' ' => {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(24.)).with_translation(Vec3::from_array([25.0 * i as f32, 25.0 * j as f32, 0.0])),
                        material: materials.add(ColorMaterial::from(Color::GREEN)),
                        ..default()
                    });
                },
                _ => println!("{}", cell),
            }            
        }
    }
}