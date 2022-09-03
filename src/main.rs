mod text;

use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};
use bevy::input::keyboard::KeyboardInput;
use bevy::text::Text2dBounds;
use crate::text::TextPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(
            WindowDescriptor {
                title: "OGRE".to_string(),
                width: 640.,
                height: 480.,
                position: WindowPosition::At(Vec2::new(5., 30.)),
                ..default()
            }
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(TextPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}