use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(
            // look at documentation
            // go to the source code
            WindowDescriptor {
                title: "OGRE".to_string(),
                width: 640.,
                height: 480.,
                position: WindowPosition::At(Vec2::new(5., 30.)),
                ..default()
            }
        )
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .run();
}
