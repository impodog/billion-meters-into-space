use bevy::input::touch::TouchPhase;

use super::*;

pub(super) fn show_ending_msg(
    mut commands: Commands,
    font: Res<DefaultFont>,
    stat: Res<GlobalStat>,
    mut save: ResMut<Save>,
) {
    let style = TextStyle {
        font: font.0.clone(),
        font_size: 100.0,
        color: Color::WHITE,
    };
    let style_big = TextStyle {
        font: font.0.clone(),
        font_size: 60.0,
        color: Color::CYAN,
    };
    let mut sections = vec![
        TextSection::new("Game Over", style_big.clone()),
        TextSection::new("\nPress R or Click to restart", style.clone()),
        TextSection::new(
            format!("\nYou traveled {} meters in space!", stat.distance),
            style.clone(),
        ),
    ];
    if stat.distance > save.high_distance {
        sections.push(TextSection::new(
            "\nNew High Score!",
            TextStyle {
                font: font.0.clone(),
                font_size: 80.0,
                color: Color::RED,
            },
        ));
        save.high_distance = stat.distance;
    } else {
        sections.push(TextSection::new(
            format!("\nHigh Score: {} meters", save.high_distance),
            style.clone(),
        ));
    }
    commands.spawn(Text2dBundle {
        text: Text::from_sections(sections),
        ..Default::default()
    });
    warn!("Game Over");
}

pub(super) fn remove_ending_msg(mut commands: Commands, q: Query<Entity, With<Text>>) {
    q.iter().for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

pub(super) fn test_restart(
    cur_state: Res<State<Status>>,
    mut state: ResMut<NextState<Status>>,
    key: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut e_touch: EventReader<TouchInput>,
) {
    let mut click = false;
    let mut swipe = false;
    if mouse.just_pressed(MouseButton::Left) {
        click = true;
    }
    for touch in e_touch.read() {
        match touch.phase {
            TouchPhase::Moved => {
                swipe = true;
            }
            TouchPhase::Ended => {
                if !swipe {
                    click = true;
                }
            }
            _ => {}
        }
    }

    if key.just_pressed(KeyCode::KeyR) || click || swipe {
        let next = match cur_state.get() {
            Status::Over => Status::Play,
            _ if !click => Status::Over,
            _ => {
                return;
            }
        };
        state.set(next);
    }
}
