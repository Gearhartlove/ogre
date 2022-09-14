use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

pub struct TextPlugin;

#[derive(Component)]
pub struct MainText;
pub struct ExecuteEvent;
struct ControlDown(bool);
pub struct CurrentLine(pub(crate) usize);

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SayEvent>()
            .add_event::<ExecuteEvent>()
            .insert_resource(ControlDown(false))
            .insert_resource(CurrentLine(0))
            .insert_resource(LineStart::default())
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system_to_stage(CoreStage::PostUpdate, type_to_screen)
            .add_system_to_stage(CoreStage::Last, render_text)
            .add_system(hold_control);
    }
}

pub struct LineStart {
    // pub const PROMPT: &'static str = "root@ogre location % ";
    pub user: &'static str,
    pub location: &'static str,
    pub prompt: &'static str,
}

impl Default for LineStart {
    fn default() -> Self {
        Self {
            user: "user",
            location: "location",
            prompt: "% ",
        }
    }
}

fn setup(mut commands: Commands, ass: Res<AssetServer>, line_start: Res<LineStart>) {
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
    commands
        .spawn_bundle(
            TextBundle::from_section(
                format!(
                    "{}@ogre {} {}",
                    line_start.user, line_start.location, line_start.prompt
                ),
                text_style,
            )
            .with_text_alignment(text_alignment)
            .with_style(style),
        )
        .insert(MainText);
}

fn type_to_screen(
    mut key_evr: EventReader<ReceivedCharacter>,
    mut exe_evw: EventWriter<ExecuteEvent>,
    mut text_query: Query<&mut Text, With<MainText>>,
    control_down: Res<ControlDown>,
    mut curr_line: ResMut<CurrentLine>,
    ass: Res<AssetServer>,
    line_start: Res<LineStart>,
) {
    for ev in key_evr.iter() {
        let c: char = ev.char;
        println!("Got char: '{}'", c);
        if let Ok(mut text) = text_query.get_single_mut() {
            // enter a command
            // if c == '
            if c == '\r' {
                let text_style = TextStyle {
                    font: ass.load("fonts/monofontorg.otf"),
                    font_size: 20.0,
                    color: Color::LIME_GREEN,
                };
                // // todo: print console after everything has been said // a stack would do nicely here
                // text.sections.push(TextSection {
                //     value: format!("\n{}", format!("{}@ogre {} {}", line_start.user, line_start.location, line_start.prompt)),
                //     style: text_style,
                // });
                // send event off to compiler
                exe_evw.send(ExecuteEvent);
            }
            // delete line
            else if c == '' {
                loop {
                    if text.sections[curr_line.0].value.pop().unwrap() == '%' {
                        text.sections[curr_line.0].value.push_str("% ");
                        break;
                    }
                }
            } else if c == '' {
                // can't delete the prompt
                let len = text.sections[curr_line.0].value.len();
                let len = len - 2;
                let slice = &text.sections[curr_line.0].value[len..];
                if slice != "% " {
                    // if holding control, delete word
                    if control_down.0 {
                        text.sections[curr_line.0].value.pop();
                        loop {
                            if text.sections[curr_line.0].value.pop().unwrap() == ' ' {
                                text.sections[curr_line.0].value.push(' ');
                                break;
                            }
                        }
                    } else {
                        // else, delete a single character
                        text.sections[curr_line.0].value.pop();
                    }
                }
            } else {
                text.sections[curr_line.0].value.push(c);
            }
        }
    }
}

fn hold_control(keys: Res<Input<KeyCode>>, mut control: ResMut<ControlDown>) {
    if keys.just_pressed(KeyCode::LControl)
        || keys.just_pressed(KeyCode::RControl)
        || keys.just_pressed(KeyCode::LWin)
    {
        control.0 = true;
    }
    if keys.just_released(KeyCode::LControl)
        || keys.just_released(KeyCode::RControl)
        || keys.just_released(KeyCode::LWin)
    {
        control.0 = false;
    }
}

pub struct SayEvent(pub String);

fn render_text(
    ass: Res<AssetServer>,
    mut text_query: Query<&mut Text, With<MainText>>,
    mut line_start: Res<LineStart>,
    mut say_reader: EventReader<SayEvent>,
    mut curr_line: ResMut<CurrentLine>,
) {
    for say in say_reader.iter() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let text_style = TextStyle {
                font: ass.load("fonts/monofontorg.otf"),
                font_size: 20.0,
                color: Color::LIME_GREEN,
            };

            text.sections.push(TextSection {
                value: say.0.to_string(),
                style: text_style.clone(),
            });

            text.sections.push(TextSection {
                value: format!(
                    "\n{}@ogre {} {}",
                    line_start.user, line_start.location, line_start.prompt
                ),
                style: text_style,
            });

            curr_line.0 += 2;
        }
    }
}
