use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Start,
    Playing,
    GameOver,
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
    for (camera_entity, camera_transform) in camera_query.iter_mut() {
        commands.entity(camera_entity).despawn();
    }
    for (player_entity, player_transform) in player_query.iter_mut() {
        commands.entity(player_entity).despawn();
    }
    for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
        commands.entity(enemy_entity).despawn();
    }
    for (player_shot_entity, player_shot_transform) in player_shot_query.iter_mut() {
        commands.entity(player_shot_entity).despawn();
    }
    for (enemy_shot_entity, enemy_shot_transform) in enemy_shot_query.iter_mut() {
        commands.entity(enemy_shot_entity).despawn();
    }
}

fn show_score(score: Res<Score>) {
    println!("Score: {}", score.0);
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
    for (player_shot_entity, player_shot_transform) in player_shot_query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            let collision = collide(
                player_shot_transform.translation,
                Vec2::new(10.0, 10.0),
                enemy_transform.translation,
                Vec2::new(50.0, 50.0),
            );
            if let Some(collision) = collision {
                println!("Collision detected: {:?}", collision);
                // delete enemy
                commands.entity(enemy_entity).despawn();
                // update score
                score.0 += 1;
            }
        }
    }
    // check for enemy shot collisions
    for (enemy_shot_entity, enemy_shot_transform) in enemy_shot_query.iter_mut() {
        for (player_entity, player_transform) in player_query.iter_mut() {
            let collision = collide(
                enemy_shot_transform.translation,
                Vec2::new(10.0, 10.0),
                player_transform.translation,
                Vec2::new(100.0, 100.0),
            );
            if let Some(collision) = collision {
                println!("Collision detected: {:?}", collision);
            }
        }
    }
    // check for enemy collisions
    for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
        for (player_entity, player_transform) in player_query.iter_mut() {
            let collision = collide(
                enemy_transform.translation,
                Vec2::new(50.0, 50.0),
                player_transform.translation,
                Vec2::new(100.0, 100.0),
            );
            if let Some(collision) = collision {
                println!("Collision detected: {:?}", collision);
                // change state to game over
                next_state.set(GameState::GameOver);
            }
        }
    }
}

fn check_state(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    println!("check_state");
    println!("{:?}", state.get());
}

fn update_state(mut next_state: ResMut<NextState<GameState>>) {
    println!("update_state");
    println!("{:?}", next_state);
    next_state.set(GameState::Playing)
}

fn continue_from_game_over(
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
    commands
        .spawn(NodeBundle {
            style: Style { ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(100.0),
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
        });
}

fn delete_start_menu(
    mut camera_query: Query<(Entity, &Transform), With<Camera>>,
    mut menu_query: Query<(Entity, &Transform), With<Node>>,
    mut commands: Commands,
) {
    for (camera_entity, camera_transform) in camera_query.iter_mut() {
        commands.entity(camera_entity).despawn();
    }
    for (menu_entity, menu_transform) in menu_query.iter_mut() {
        commands.entity(menu_entity).despawn();
    }
}

fn start_game(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn result_menu(mut commands: Commands) {
    // ui camera
    // commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style { ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(100.0),
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
                });
        });
}

fn delete_result_menu(
    mut camera_query: Query<(Entity, &Transform), With<Camera>>,
    mut menu_query: Query<(Entity, &Transform), With<Node>>,
    mut commands: Commands,
) {
    for (camera_entity, camera_transform) in camera_query.iter_mut() {
        commands.entity(camera_entity).despawn();
    }
    for (menu_entity, menu_transform) in menu_query.iter_mut() {
        commands.entity(menu_entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(OnEnter(GameState::Start), start_menu)
        .add_systems(Update, start_game.run_if(in_state(GameState::Start)))
        .add_systems(OnExit(GameState::Start), delete_start_menu)
        .add_systems(OnEnter(GameState::Playing), (setup))
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
        .add_systems(OnEnter(GameState::GameOver), result_menu)
        .add_systems(
            Update,
            (continue_from_game_over).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnExit(GameState::GameOver),
            (delete_all, delete_result_menu),
        )
        // .add_systems(Update, show_score)
        .run();
}
