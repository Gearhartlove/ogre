use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ControlDown(false))
            .add_startup_system(setup)
            .add_system(type_to_screen)
            .add_system(hold_control);
    }
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(3.4, 1.2, 0.).looking_at(Vec3::new(0., 1.2, 0.), Vec3::Y),
        ..default()
    });
    // tv
    commands.spawn_bundle(SceneBundle {
        scene: ass.load("3d/tv.gltf#Scene0"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 0.0),
        ..default()
    });
    // text
    let font = ass.load("fonts/monofontorg.otf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::LIME_GREEN,
    };
    let style = Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        max_size: Size::new(Val::Px(500.), Val::Px(340.)),
        position: UiRect {
            top: Val::Px(70.0),
            left: Val::Px(78.0),
            ..default()
        },
        ..default()
    };
    let text_alignment = TextAlignment::TOP_LEFT;
    commands.spawn_bundle(TextBundle::from_section(
        "> |",
        text_style,
    )
        .with_text_alignment(text_alignment)
        .with_style(style))
        .insert(MainText);
}

#[derive(Component)]
struct MainText;

fn type_to_screen(
    mut key_evr: EventReader<ReceivedCharacter>,
    mut text_query: Query<&mut Text, With<MainText>>,
    control_down: Res<ControlDown>,
) {
    for ev in key_evr.iter() {
        let c = ev.char;
        println!("Got char: '{}'", c);
        if let Ok(mut text) = text_query.get_single_mut() {
            // delete line
            if c == '' {
                loop {
                    if text.sections[0].value.pop().unwrap() == '>' {
                        text.sections[0].value.push_str("> ");
                        break;
                    }
                }
            }
            // delete word
            if c == '' {
                // if holding control, delete word
                if control_down.0 {
                    text.sections[0].value.pop();
                    loop {
                        if text.sections[0].value.pop().unwrap() == ' ' {
                            text.sections[0].value.push(' ');
                            break;
                        }
                    }
                } else {
                    // else, delete a single character
                    text.sections[0].value.pop();
                }
            } else {
                text.sections[0].value.push(c);
            }
        }
    }
}

struct ControlDown(bool);

fn hold_control(
    keys: Res<Input<KeyCode>>,
    mut control: ResMut<ControlDown>,
) {
    if keys.just_pressed(KeyCode::LControl)
        || keys.just_pressed(KeyCode::RControl)
        || keys.just_pressed(KeyCode::LWin) {
        control.0 = true;
    }
    if keys.just_released(KeyCode::LControl)
        || keys.just_released(KeyCode::RControl)
        || keys.just_released(KeyCode::LWin) {
        control.0 = false;
    }
}
