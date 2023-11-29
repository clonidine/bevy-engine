use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Query, Res},
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    render::texture::ImagePlugin,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let _ = &characters.iter_mut().for_each(|(mut transform, _)| {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        } else if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        } else if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        } else if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Duck Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run()
}
