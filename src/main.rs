mod plugins;
mod utils;

use bevy::prelude::*;
use plugins::LoaderPlugin;

#[derive(Resource, Default, Debug)]
pub struct GameAssets {
    ferris: Handle<Image>,
}

#[derive(Component, Debug)]
struct Ferris;

#[derive(Component, Default, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let ferris_system_set = SystemSet::new()
        .with_system(sys_apply_ferris_gravity)
        .with_system(sys_apply_ferris_velocity)
        .with_system(sys_apply_ferris_rotation)
        .with_system(sys_handle_ferris_input);

    App::new()
        .add_plugin(LoaderPlugin)
        .add_plugins(utils::modify_default_plugins())
        .add_startup_system_to_stage(StartupStage::PostStartup, sys_spawn_ferris)
        .add_system_set(ferris_system_set)
        .run();
}

fn sys_spawn_ferris(mut cmds: Commands, game_assets: Res<GameAssets>) {
    const FERRIS_SPRITE_SCALE: f32 = 0.15;

    let ferris_bundle = (
        SpriteBundle {
            texture: game_assets.ferris.clone(),
            transform: Transform {
                scale: Vec3::new(FERRIS_SPRITE_SCALE, FERRIS_SPRITE_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        Ferris,
        Velocity::default(),
    );

    cmds.spawn(ferris_bundle);
}

fn sys_apply_ferris_gravity(mut q: Query<&mut Velocity>) {
    const GRAVITY: f32 = 9.8;

    let mut velocity = q.single_mut();
    velocity.y -= GRAVITY;
}

fn sys_apply_ferris_velocity(
    mut q: Query<(&Velocity, &mut Transform), With<Ferris>>,
    time: Res<Time>,
) {
    let (velocity, mut transform) = q.single_mut();

    transform.translation.x += velocity.x * time.delta_seconds();
    transform.translation.y += velocity.y * time.delta_seconds();
}

fn sys_handle_ferris_input(kb: Res<Input<KeyCode>>, mut q: Query<&mut Velocity, With<Ferris>>) {
    const FERRIS_JUMP_POWER: f32 = 225.;

    if let Ok(mut velocity) = q.get_single_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velocity.y = FERRIS_JUMP_POWER;
        }
    }
}

fn sys_apply_ferris_rotation(
    mut q: Query<(&Velocity, &mut Transform), With<Ferris>>,
    time: Res<Time>,
) {
    let (velocity, mut transform) = q.single_mut();
    let rotation_sign = f32::copysign(1.0, velocity.y);
    let rotation_angle = rotation_sign * 180.;

    transform.rotate_z(f32::to_radians(rotation_angle) * time.delta_seconds());
}
