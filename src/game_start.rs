use bevy::prelude::*;

use crate::game::GameState;

pub struct GameStartPlugin;

#[derive(Component, Debug)]
enum StartMenu {
    Start,
    Exit,
}

fn start_menu(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn(StartMenu::Start);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
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
                        "Start",
                        TextStyle {
                            font_size: 50.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
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
                    // border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font_size: 50.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}

fn input_start_menu(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut start_menu_query: Query<&mut StartMenu>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for mut start_menu in start_menu_query.iter_mut() {
            *start_menu = StartMenu::Start;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        for mut start_menu in start_menu_query.iter_mut() {
            *start_menu = StartMenu::Exit;
        }
    }
    if keyboard_input.just_pressed(KeyCode::Return) {
        for start_menu in start_menu_query.iter_mut() {
            match *start_menu {
                StartMenu::Start => {
                    next_state.set(GameState::Playing);
                }
                StartMenu::Exit => {
                    std::process::exit(0);
                }
            }
        }
    }
}

fn update_start_menu(
    mut menu_button_query: Query<(&mut BorderColor, &Children), With<Button>>,
    text_query: Query<&Text>,
    start_menu_query: Query<&StartMenu>,
) {
    let start_menu = start_menu_query.get_single();
    if start_menu.is_err() {
        return;
    }
    match start_menu.unwrap() {
        StartMenu::Start => {
            for (mut border_color, children) in menu_button_query.iter_mut() {
                for child in children.iter() {
                    let text = text_query.get(*child).unwrap();
                    if text.sections.get(0).unwrap().value.as_str() == "Start" {
                        border_color.0 = Color::RED;
                    } else {
                        border_color.0 = Color::BLACK;
                    }
                }
            }
        }
        StartMenu::Exit => {
            for (mut border_color, children) in menu_button_query.iter_mut() {
                for child in children.iter() {
                    let text = text_query.get(*child).unwrap();
                    if text.sections.get(0).unwrap().value.as_str() == "Exit" {
                        border_color.0 = Color::RED;
                    } else {
                        border_color.0 = Color::BLACK;
                    }
                }
            }
        }
    }
}

fn delete_start_menu(
    mut camera_query: Query<(Entity, &Transform), With<Camera>>,
    mut start_menu_query: Query<Entity, With<StartMenu>>,
    mut menu_query: Query<(Entity, &Transform), With<Node>>,
    mut commands: Commands,
) {
    for (camera_entity, _camera_transform) in camera_query.iter_mut() {
        commands.entity(camera_entity).despawn();
    }
    for start_menu_entity in start_menu_query.iter_mut() {
        commands.entity(start_menu_entity).despawn();
    }
    for (menu_entity, _menu_transform) in menu_query.iter_mut() {
        commands.entity(menu_entity).despawn();
    }
}

impl Plugin for GameStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), start_menu)
            .add_systems(
                Update,
                (update_start_menu, input_start_menu).run_if(in_state(GameState::Start)),
            )
            .add_systems(OnExit(GameState::Start), delete_start_menu);
    }
}
