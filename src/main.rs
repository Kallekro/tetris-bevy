use bevy::{prelude::*, math::vec3};
use rand::prelude::*;

const BLOCK_SIZE: f32 = 10.0;
const TICK: f32 = 0.1;
const MOVE_TIMEOUT: f32 = 0.2;
const ARENA_WIDTH: f32 = 200.0;
const ARENA_HEIGHT: f32 = 200.0;
const ARENA_START: Vec2 = Vec2::new(-100.0, 0.0);

const COLORS: [Color; 7] = [
    Color::rgb(0.5, 0.5, 1.0),
    Color::rgb(0.5, 1.0, 0.5),
    Color::rgb(1.0, 0.5, 0.5),
    Color::rgb(1.0, 1.0, 0.5),
    Color::rgb(1.0, 0.5, 1.0),
    Color::rgb(0.5, 1.0, 1.0),
    Color::rgb(1.0, 0.5, 0.5),
];

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(TickTimer(Timer::from_seconds(TICK, TimerMode::Repeating)))
    .insert_resource(MoveTimeout(Timer::from_seconds(MOVE_TIMEOUT, TimerMode::Once)))
    .insert_resource(RotateTimeout(Timer::from_seconds(MOVE_TIMEOUT, TimerMode::Once)))
    .insert_resource(AllowLeft(true))
    .insert_resource(AllowRight(true))
    .add_systems(Startup, (setup, spawn_first_piece).chain())
    .add_systems(FixedUpdate, (check_movement, move_piece, fall_down).chain())
    .run();
}

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Resource)]
struct MoveTimeout(Timer);

#[derive(Resource)]
struct RotateTimeout(Timer);

#[derive(Resource)]
struct AllowLeft(bool);

#[derive(Resource)]
struct AllowRight(bool);

#[derive(Component)]
struct ActivePiece;

#[derive(Component)]
struct Block {
    pivot: u32
}

#[derive(Bundle)]
struct BlockBundle {
    block: Block,
    sprite: SpriteBundle,
}

impl BlockBundle {
    fn new(x: f32, y: f32, color: Color, pivot: u32) -> Self {
        Self {
            block: Block { pivot },
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: vec3(BLOCK_SIZE, BLOCK_SIZE, 0.0) * 0.9,
                    ..default()
                },
                sprite: Sprite {
                    color: color,
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl ActivePiece {
    fn spawn_o(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color, 0), ActivePiece));
    }

    fn spawn_i(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color, 0), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 3.0, color, 0), ActivePiece));
    }

    fn spawn_s(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
    }

    fn spawn_z(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y, color, 1), ActivePiece));
    }

    fn spawn_l(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE * 2.0, color, 1), ActivePiece));
    }

    fn spawn_j(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE * 2.0, color, 1), ActivePiece));
    }

    fn spawn_t(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y, color, 1), ActivePiece));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color, 1), ActivePiece));
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_rand_piece(
    commands: Commands,
) {
    let piece_start: Vec2 = ARENA_START + Vec2::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT - BLOCK_SIZE);
    let mut rng = rand::thread_rng();
    let piece_type = rng.gen_range(0..7);
    let color = COLORS[piece_type];

    match piece_type {
        0 => ActivePiece::spawn_o(commands, color, piece_start),
        1 => ActivePiece::spawn_i(commands, color, piece_start),
        2 => ActivePiece::spawn_s(commands, color, piece_start),
        3 => ActivePiece::spawn_z(commands, color, piece_start),
        4 => ActivePiece::spawn_l(commands, color, piece_start),
        5 => ActivePiece::spawn_j(commands, color, piece_start),
        6 => ActivePiece::spawn_t(commands, color, piece_start),
        _ => ActivePiece::spawn_o(commands, color, piece_start),
    }
}

fn spawn_first_piece(
    commands: Commands,
) {
    let piece_start: Vec2 = ARENA_START + Vec2::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT - BLOCK_SIZE);
    let color = COLORS[0];
    ActivePiece::spawn_o(commands, color, piece_start);
}

fn move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<ActivePiece>>,
    time: Res<Time>,
    mut move_timeout: ResMut<MoveTimeout>,
    mut rotate_timeout: ResMut<RotateTimeout>,
    allow_left: Res<AllowLeft>,
    allow_right: Res<AllowRight>,
) {
    let mut move_left = false;
    let mut move_right = false;
    if move_timeout.0.tick(time.delta()).finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            move_left = allow_left.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            move_right = allow_right.0;
        }
        if move_left || move_right {
            move_timeout.0.reset();
        }
    }

    if rotate_timeout.0.tick(time.delta()).finished() {
        if keyboard_input.pressed(KeyCode::Up) {
            let pivot = query.iter().nth(0).unwrap().translation;
            let rotation = Quat::from_rotation_z(90.0f32.to_radians());
            for mut transform in query.iter_mut() {
                transform.rotate_around(pivot, rotation);
            }
            rotate_timeout.0.reset();
        }
    }

    /* let pivot = query.iter().nth(0).unwrap().1.translation;
    let rotation = Quat::from_rotation_z(90.0f32.to_radians()); */

    for mut transform in query.iter_mut() {
        if move_left {
            transform.translation.x -= BLOCK_SIZE;
        }
        if move_right {
            transform.translation.x += BLOCK_SIZE;
        }

        //transform.rotate_around(pivot, rotation);
    }
}

fn check_movement(
    mut active_blocks_query: Query<&Transform, With<ActivePiece>>,
    mut inactive_blocks_query: Query<&Transform, (With<Block>, Without<ActivePiece>)>,
    mut allow_left: ResMut<AllowLeft>,
    mut allow_right: ResMut<AllowRight>,
) {
    allow_left.0 = true;
    allow_right.0 = true;
    for active_transform in active_blocks_query.iter_mut() {
        // Check if active block is at edge of arena
        if active_transform.translation.x == ARENA_START.x {
            allow_left.0 = false;
        }
        if active_transform.translation.x == ARENA_START.x + ARENA_WIDTH - BLOCK_SIZE {
            allow_right.0 = false;
        }

        for inactive_transform in inactive_blocks_query.iter_mut() {
            // Check if active block is next to inactive block
            if active_transform.translation.y == inactive_transform.translation.y {
                if active_transform.translation.x - BLOCK_SIZE == inactive_transform.translation.x {
                    allow_left.0 = false;
                }
                if active_transform.translation.x + BLOCK_SIZE == inactive_transform.translation.x {
                    allow_right.0 = false;
                }
            }
        }
    }
}

fn fall_down(
    mut commands: Commands,
    mut active_blocks_query: Query<(Entity, &mut Transform), With<ActivePiece>>,
    mut inactive_blocks_query: Query<&Transform, (With<Block>, Without<ActivePiece>)>,
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
) {
    if !tick_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let mut still_active = true;

    for (_, active_transform) in active_blocks_query.iter_mut() {
        for inactive_transform in inactive_blocks_query.iter_mut() {
            // Check if active block is above inactive block before falling
            if active_transform.translation.y - BLOCK_SIZE == inactive_transform.translation.y && active_transform.translation.x == inactive_transform.translation.x {
                still_active = false;
                break;
            }
        }
        // Check if active block is at bottom of arena before falling
        if active_transform.translation.y - BLOCK_SIZE <= ARENA_START.y - ARENA_HEIGHT {
            still_active = false;
        }
        if !still_active {
            break;
        }
    }

    if !still_active {
        for (entity, _) in active_blocks_query.iter_mut() {
            commands.entity(entity).remove::<ActivePiece>();
        }
        spawn_rand_piece(commands);
    } else {
        for (_, mut transform) in active_blocks_query.iter_mut() {
            transform.translation.y -= 10.0;
        }
    }
}