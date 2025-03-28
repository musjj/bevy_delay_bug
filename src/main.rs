use bevy::{color::palettes::css::PINK, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera2d,
                Camera {
                    clear_color: ClearColorConfig::Custom(PINK.into()),
                    ..default()
                },
            ));
        })
        .run()
}
