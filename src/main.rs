use bevy::{prelude::*, math::vec3};
use rand::prelude::*;

const BLOCK_SIZE: f32 = 10.0;
const TICK: f32 = 0.1;
const MOVE_TIMEOUT: f32 = 0.1;
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
struct AllowLeft(bool);

#[derive(Resource)]
struct AllowRight(bool);

#[derive(Component)]
struct Active;

#[derive(Component)]
struct Piece;

#[derive(Component)]
struct Block;

#[derive(Bundle)]
struct BlockBundle {
    block: Block,
    sprite: SpriteBundle,
}

impl BlockBundle {
    fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            block: Block,
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

impl Active {
    fn spawn_o(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color), Active));
    }

    fn spawn_i(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 3.0, color), Active));
    }

    fn spawn_s(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y + BLOCK_SIZE, color), Active));
    }

    fn spawn_z(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y, color), Active));
    }

    fn spawn_l(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE * 2.0, color), Active));
    }

    fn spawn_j(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y + BLOCK_SIZE * 2.0, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE * 2.0, color), Active));
    }

    fn spawn_t(mut commands: Commands, color : Color, spawn_pos: Vec2) {
        commands.spawn((BlockBundle::new(spawn_pos.x, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE * 2.0, spawn_pos.y, color), Active));
        commands.spawn((BlockBundle::new(spawn_pos.x + BLOCK_SIZE, spawn_pos.y + BLOCK_SIZE, color), Active));
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
        0 => Active::spawn_o(commands, color, piece_start),
        1 => Active::spawn_i(commands, color, piece_start),
        2 => Active::spawn_s(commands, color, piece_start),
        3 => Active::spawn_z(commands, color, piece_start),
        4 => Active::spawn_l(commands, color, piece_start),
        5 => Active::spawn_j(commands, color, piece_start),
        6 => Active::spawn_t(commands, color, piece_start),
        _ => Active::spawn_o(commands, color, piece_start),
    }
}

fn spawn_first_piece(
    commands: Commands,
) {
    let piece_start: Vec2 = ARENA_START + Vec2::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT - BLOCK_SIZE);
    let color = COLORS[0];
    Active::spawn_o(commands, color, piece_start);
}

fn move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Transform), With<Active>>,
    time: Res<Time>,
    mut move_timeout: ResMut<MoveTimeout>,
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

    for (_, mut transform) in query.iter_mut() {
        if move_left {
            transform.translation.x -= BLOCK_SIZE;
        }
        if move_right {
            transform.translation.x += BLOCK_SIZE;
        }
    }
}

fn check_movement(
    mut active_blocks_query: Query<&Transform, With<Active>>,
    mut inactive_blocks_query: Query<&Transform, (With<Block>, Without<Active>)>,
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
    mut active_blocks_query: Query<(Entity, &mut Transform), With<Active>>,
    mut inactive_blocks_query: Query<&Transform, (With<Block>, Without<Active>)>,
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
            commands.entity(entity).remove::<Active>();
        }
        spawn_rand_piece(commands);
    } else {
        for (_, mut transform) in active_blocks_query.iter_mut() {
            transform.translation.y -= 10.0;
        }
    }
}