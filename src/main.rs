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

#[derive(Component, Debug)]
struct RotateWhileFalling;

fn main() {
    App::new()
        .add_plugin(LoaderPlugin)
        .add_plugins(utils::modify_default_plugins())
        .add_startup_system_to_stage(StartupStage::PostStartup, sys_spawn_ferris)
        .add_system(sys_apply_gravity)
        .add_system(sys_apply_velocity)
        .add_system(sys_input)
        .add_system(sys_apply_rotation)
        .add_system(bevy::window::close_on_esc)
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
        RotateWhileFalling,
    );

    cmds.spawn(ferris_bundle);
}

fn sys_apply_gravity(mut q: Query<&mut Velocity>) {
    for mut velocity in q.iter_mut() {
        velocity.y -= 8.;
    }
}

fn sys_apply_velocity(mut q: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in q.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn sys_input(kb: Res<Input<KeyCode>>, mut q: Query<&mut Velocity, With<Ferris>>) {
    const FERRIS_JUMP_POWER: f32 = 225.;

    if let Ok(mut velocity) = q.get_single_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velocity.y = FERRIS_JUMP_POWER;
        }
    }
}

fn sys_apply_rotation(
    mut q: Query<(&Velocity, &mut Transform), With<RotateWhileFalling>>,
    time: Res<Time>,
) {
    let rotation_speed = f32::to_radians(360.);
    for (velocity, mut transform) in q.iter_mut() {
        let rotation_angle = rotation_speed * time.delta_seconds();
        let rotation_sign = f32::copysign(1.0, velocity.y);

        transform.rotate_z(rotation_sign * rotation_angle);
    }
}
