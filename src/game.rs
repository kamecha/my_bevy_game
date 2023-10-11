use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct PlayerShot;

#[derive(Component)]
struct EnemyShot;

#[derive(Component)]
struct Collider;

#[derive(Resource)]
struct Score(usize);

#[derive(Component, Debug)]
struct ScoreUI;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Start,
    Playing,
    Result,
}

#[derive(Component, Debug)]
enum StartMenu {
    Start,
    Exit,
}

#[derive(Component, Debug)]
enum ResultMenu {
    Restart,
    BackToTitle,
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(0.0, 0.0),
                    max: Vec2::new(100.0, 100.0),
                }),
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
    ));
    // Score
    commands.insert_resource(Score(0));
    // Score UI
    commands
        .spawn((
            NodeBundle {
                style: Style { ..default() },
                ..default()
            },
            ScoreUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Score: ".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "0".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        });
}

fn create_enemy(mut commands: Commands) {
    let hoge: i32 = rand::thread_rng().gen_range(0, 100);
    // 一定周期で敵を生成
    if hoge == 0 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(50.0, 50.0),
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(rand::thread_rng().gen_range(-300.0, 300.0), 300.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Enemy,
            Collider,
        ));
    }
}

fn create_player_shot(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
) {
    let player_transform = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        // shot
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(10.0, 10.0),
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: player_transform.translation,
                    ..default()
                },
                ..default()
            },
            PlayerShot,
            Collider,
        ));
    }
}

fn create_enemy_shot(mut query: Query<&mut Transform, With<Enemy>>, mut commands: Commands) {
    // 一定周期でショットを生成
    if rand::thread_rng().gen_range(0, 100) != 0 {
        return;
    }
    for enemy_transform in query.iter_mut() {
        // shot
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(10.0, 10.0),
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: enemy_transform.translation,
                    ..default()
                },
                ..default()
            },
            EnemyShot,
            Collider,
        ));
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;
    let speed = 800.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= Vec3::Y;
    }

    player_transform.translation += time_step.period.as_secs_f32() * direction * speed;
}

fn move_shot(mut query: Query<&mut Transform, With<PlayerShot>>, time_step: Res<FixedTime>) {
    for mut shot_transform in query.iter_mut() {
        shot_transform.translation += time_step.period.as_secs_f32() * Vec3::Y * 1000.0;
    }
}

fn move_enemy_shot(mut query: Query<&mut Transform, With<EnemyShot>>, time_step: Res<FixedTime>) {
    for mut shot_transform in query.iter_mut() {
        shot_transform.translation -= time_step.period.as_secs_f32() * Vec3::Y * 500.0;
    }
}

fn move_enemy(mut query: Query<&mut Transform, With<Enemy>>, time_step: Res<FixedTime>) {
    for mut enemy_transform in query.iter_mut() {
        // 波状に下へ移動
        enemy_transform.translation -= time_step.period.as_secs_f32() * Vec3::Y * 100.0;
    }
}

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

fn show_score(
    score: Res<Score>,
    mut score_ui_query: Query<&mut Children, With<ScoreUI>>,
    mut commands: Commands,
) {
    // update score ui
    let score_text: String = score.0.to_string();
    for children in score_ui_query.iter_mut() {
        for child in children.iter() {
            let mut text = Text::default();
            text.sections = vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font_size: 50.0,
                        color: Color::BLACK,
                        ..default()
                    },
                },
                TextSection {
                    value: score_text.clone(),
                    style: TextStyle {
                        font_size: 50.0,
                        color: Color::BLACK,
                        ..default()
                    },
                },
            ];
            commands.entity(*child).insert(text);
        }
    }
}

fn check_for_collisions(
    mut score: ResMut<Score>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut player_shot_query: Query<(Entity, &Transform), With<PlayerShot>>,
    mut enemy_shot_query: Query<(Entity, &Transform), With<EnemyShot>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    // check for player shot collisions
    for (_player_shot_entity, player_shot_transform) in player_shot_query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            let collision = collide(
                player_shot_transform.translation,
                Vec2::new(10.0, 10.0),
                enemy_transform.translation,
                Vec2::new(50.0, 50.0),
            );
            if let Some(collision) = collision {
                debug!("Collision detected: {:?}", collision);
                // delete enemy
                commands.entity(enemy_entity).despawn();
                // update score
                score.0 += 1;
            }
        }
    }
    // check for enemy shot collisions
    for (_enemy_shot_entity, enemy_shot_transform) in enemy_shot_query.iter_mut() {
        for (_player_entity, player_transform) in player_query.iter_mut() {
            let collision = collide(
                enemy_shot_transform.translation,
                Vec2::new(10.0, 10.0),
                player_transform.translation,
                Vec2::new(100.0, 100.0),
            );
            if let Some(collision) = collision {
                debug!("Collision detected: {:?}", collision);
            }
        }
    }
    // check for enemy collisions
    for (_enemy_entity, enemy_transform) in enemy_query.iter_mut() {
        for (_player_entity, player_transform) in player_query.iter_mut() {
            let collision = collide(
                enemy_transform.translation,
                Vec2::new(50.0, 50.0),
                player_transform.translation,
                Vec2::new(100.0, 100.0),
            );
            if let Some(collision) = collision {
                debug!("Collision detected: {:?}", collision);
                next_state.set(GameState::Result);
            }
        }
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            GameStartPlugin,
            GamePlayingPlugin,
            GameResultPlugin,
        ));
    }
}

pub struct GameStartPlugin;

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

pub struct GamePlayingPlugin;

impl Plugin for GamePlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                FixedUpdate,
                (
                    move_player,
                    move_shot,
                    create_player_shot,
                    create_enemy,
                    create_enemy_shot,
                    move_enemy,
                    move_enemy_shot,
                    check_for_collisions,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, show_score.run_if(in_state(GameState::Playing)));
    }
}

pub struct GameResultPlugin;

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
