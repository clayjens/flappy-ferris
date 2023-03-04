use bevy::{log, prelude::*};

use crate::GameAssets;

/// A Plugin for loading GameAssets and preparing Rendering behaviors at Startup.
pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(sys_load_assets)
            .add_startup_system(sys_prepare_rendering);
    }
}

/// Loads assets into GameAssets resource.
fn sys_load_assets(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let mut assets = GameAssets::default();

    log::info!("Loading Assets...");

    assets.ferris = asset_server.load("rustacean-flat-happy.png");

    log::info!("Assets Loaded!");

    cmds.insert_resource(assets);
}

/// Prepares a 2D camera and other rendering-specific behaviors.
fn sys_prepare_rendering(mut cmds: Commands) {
    const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

    log::info!("Preparing Rendering System...");

    cmds.spawn(Camera2dBundle::default());
    cmds.insert_resource(ClearColor(BACKGROUND_COLOR));

    log::info!("Rendering System Prepared!");
}
