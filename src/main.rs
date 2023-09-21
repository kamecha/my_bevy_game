use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct PlayerShot;

#[derive(Component)]
struct EnemyShot;

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
    ));
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                move_shot,
                create_player_shot,
                create_enemy,
                create_enemy_shot,
                move_enemy,
                move_enemy_shot
            ),
        )
        .run();
}
