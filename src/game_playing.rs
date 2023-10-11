use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

use crate::game::GameState;

pub struct GamePlayingPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct PlayerShot;

#[derive(Component)]
pub struct EnemyShot;

#[derive(Component)]
struct Collider;

#[derive(Resource)]
struct Score(usize);

#[derive(Component, Debug)]
struct ScoreUI;

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
