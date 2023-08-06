use bevy::prelude::*;

use crate::game::GameState;

pub fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameState {
        ..Default::default()
    });

    let font = asset_server.load("fonts/font.ttf");
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "SCORE\n",
                TextStyle {
                    font_size: 10.,
                    color: Color::WHITE,
                    font: font.clone(),
                    ..default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 10.,
                    color: Color::WHITE,
                    font: font.clone(),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_text_alignment(TextAlignment::Center),
    );
}

pub fn hud_update(game_hub: Res<GameState>, mut query: Query<&mut Text>) {
    if game_hub.pill_cool_down.just_finished() {
        println!("Stay away for the ghosts");
    } else if game_hub.pill_cool_down.elapsed().as_secs() > 7 {
        println!("Time to finish up");
    }
    let mut text = query.single_mut();
    text.sections[1].value = game_hub.score.to_string();
}
