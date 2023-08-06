use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub score: usize,
    pub pill_cool_down: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: 0,
            pill_cool_down: Timer::from_seconds(0., TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct PacManChompSound(Handle<AudioSource>);

pub fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Pac-Man chomp sound
    let pacman_chomp_sound = asset_server.load("sounds/pacman_chomp.ogg");
    commands.insert_resource(PacManChompSound(pacman_chomp_sound));
}

pub fn power_pill_cool_down(time: Res<Time>, mut game_state: ResMut<GameState>) {
    game_state.pill_cool_down.tick(time.delta());

    // if game_state.pill_cool_down.finished() {
    //     commands.entity(entity).remove::<PowerPillEffect>();
    //     // The power pill effect has worn off. Do something here.
    // }
}

// fn _play_chomp_sound(
//     mut commands: Commands,
//     collision_events: EventReader<CollisionEvent>,
//     chomp_sound: Res<PacManChompSound>,
// ) {
//     if !collision_events.is_empty() {
//         commands.spawn(AudioBundle {
//             source: chomp_sound.0.clone(),
//             settings: PlaybackSettings::DESPAWN,
//         });
//     }
// }
