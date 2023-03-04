use bevy::{app::PluginGroupBuilder, prelude::*};

/// Modifies DefaultPlugins with a custom WindowPlugin.
pub fn modify_default_plugins() -> PluginGroupBuilder {
    /// Describes the WindowDescriptor configuration struct for WindowPlugin's window interface.
    fn describe_window() -> WindowDescriptor {
        WindowDescriptor {
            width: 500.,
            height: 500.,
            title: "Flappy Ferris".into(),
            resizable: false,
            ..default()
        }
    }

    /// Applies the WindowDescriptor to the WindowPlugin.
    fn prepare_window_plugin() -> WindowPlugin {
        WindowPlugin {
            window: describe_window(),
            ..default()
        }
    }

    // Builds the DefaultPlugins Plugin Group with the custom WindowPlugin.
    DefaultPlugins.set(prepare_window_plugin())
}
