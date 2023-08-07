use bevy::prelude::*;
use bevy::window::*;
use bevy_rapier2d::prelude::*;

// Shared stuff
mod constants;
use constants::*;

// Shared resources
mod shared;

// Camera(s)
mod camera;
use camera::*;

// HUD
mod hud;
use hud::*;

// Game
mod game;
use game::{power_pill_cool_down, setup_sounds};

// Ghosts
mod ghosts;
use ghosts::*;

// Pills
mod pills;
use pills::*;

// Walls
mod walls;
use walls::*;

// Player
mod player;
use player::*;

// Utils
mod utils;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        title: "Pac-Man".into(),
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_hud,
                setup_sounds,
                setup_ghosts,
                setup_pills,
                setup_walls,
                setup_player,
            ),
        )
        .add_systems(
            Update,
            (
                player_update,
                normal_pills_update,
                power_pills_update,
                ghosts_update,
                check_for_collionsion_events,
            ),
        )
        .add_systems(PostUpdate, (hud_update, move_ghosts))
        .add_systems(FixedUpdate, power_pill_cool_down)
        .run();
}
