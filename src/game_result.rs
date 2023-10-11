use bevy::prelude::*;

use crate::{
    game::GameState,
    game_playing::{Enemy, EnemyShot, Player, PlayerShot},
};

#[derive(Component, Debug)]
enum ResultMenu {
    Restart,
    BackToTitle,
}

pub struct GameResultPlugin;

fn delete_all(
    mut camera_query: Query<(Entity, &Transform), With<Camera>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut player_shot_query: Query<(Entity, &Transform), With<PlayerShot>>,
    mut enemy_shot_query: Query<(Entity, &Transform), With<EnemyShot>>,
    mut commands: Commands,
) {
    for (camera_entity, _camera_transform) in camera_query.iter_mut() {
        commands.entity(camera_entity).despawn();
    }
    for (player_entity, _player_transform) in player_query.iter_mut() {
        commands.entity(player_entity).despawn();
    }
    for (enemy_entity, _enemy_transform) in enemy_query.iter_mut() {
        commands.entity(enemy_entity).despawn();
    }
    for (player_shot_entity, _player_shot_transform) in player_shot_query.iter_mut() {
        commands.entity(player_shot_entity).despawn();
    }
    for (enemy_shot_entity, _enemy_shot_transform) in enemy_shot_query.iter_mut() {
        commands.entity(enemy_shot_entity).despawn();
    }
}

fn continue_from_result(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn result_menu(mut commands: Commands) {
    // ui camera
    // commands.spawn(Camera2dBundle::default());
    commands.spawn(ResultMenu::Restart);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Result",
                        TextStyle {
                            font_size: 50.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(100.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Restart",
                                        TextStyle {
                                            font_size: 50.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    ));
                                });
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(200.0),
                                        height: Val::Px(100.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Back to Title",
                                        TextStyle {
                                            font_size: 50.0,
                                            color: Color::BLACK,
                                            ..default()
                                        },
                                    ));
                                });
                        });
                });
        });
}

fn update_result_menu(
    mut menu_button_query: Query<(&mut BorderColor, &Children), With<Button>>,
    text_query: Query<&Text>,
    result_menu_query: Query<&ResultMenu>,
) {
    let result_menu = result_menu_query.get_single();
    if result_menu.is_err() {
        return;
    }
    match result_menu.unwrap() {
        ResultMenu::Restart => {
            for (mut border_color, children) in menu_button_query.iter_mut() {
                for child in children.iter() {
                    let text = text_query.get(*child).unwrap();
                    if text.sections.get(0).unwrap().value.as_str() == "Restart" {
                        border_color.0 = Color::RED;
                    } else {
                        border_color.0 = Color::BLACK;
                    }
                }
            }
        }
        ResultMenu::BackToTitle => {
            for (mut border_color, children) in menu_button_query.iter_mut() {
                for child in children.iter() {
                    let text = text_query.get(*child).unwrap();
                    if text.sections.get(0).unwrap().value.as_str() == "Back to Title" {
                        border_color.0 = Color::RED;
                    } else {
                        border_color.0 = Color::BLACK;
                    }
                }
            }
        }
    }
}

fn delete_result_menu(
    mut result_menu_query: Query<Entity, With<ResultMenu>>,
    mut menu_query: Query<Entity, With<Node>>,
    mut commands: Commands,
) {
    // delete result menu
    for result_menu in result_menu_query.iter_mut() {
        commands.entity(result_menu).despawn();
    }
    for menu in menu_query.iter_mut() {
        commands.entity(menu).despawn();
    }
}

fn input_result_menu(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut result_menu_query: Query<&mut ResultMenu>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for mut result_menu in result_menu_query.iter_mut() {
            *result_menu = ResultMenu::Restart;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        for mut result_menu in result_menu_query.iter_mut() {
            *result_menu = ResultMenu::BackToTitle;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Return) {
        for result_menu in result_menu_query.iter_mut() {
            match *result_menu {
                ResultMenu::Restart => {
                    next_state.set(GameState::Playing);
                }
                ResultMenu::BackToTitle => {
                    next_state.set(GameState::Start);
                }
            }
        }
    }
}

impl Plugin for GameResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Result), result_menu)
            .add_systems(
                Update,
                (continue_from_result, update_result_menu, input_result_menu)
                    .run_if(in_state(GameState::Result)),
            )
            .add_systems(OnExit(GameState::Result), (delete_all, delete_result_menu));
    }
}
