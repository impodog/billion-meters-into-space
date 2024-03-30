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
        TextSection::new("\nPress R to restart", style.clone()),
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
) {
    if key.just_pressed(KeyCode::KeyR) {
        let next = match cur_state.get() {
            Status::Over => Status::Play,
            _ => Status::Over,
        };
        state.set(next);
    }
}
