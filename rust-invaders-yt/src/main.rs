#![allow(unused)] // silence unused warnings while learning

use bevy::{
    prelude::*,
    window::{WindowDestroyed, WindowResolution},
};

const PLAYER_SPRITE: &str = "player_a_01.png";

// Entity, Component, System, Resource

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Rust Invaders!"),
                        resolution: WindowResolution::new(600.0, 680.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {}
