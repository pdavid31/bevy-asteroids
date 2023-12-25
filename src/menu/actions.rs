use bevy::{app::AppExit, prelude::*};

use crate::state::GameState;

pub(super) const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub(super) const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub(super) const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// All actions that can be triggered from a button click
#[derive(Component)]
pub(super) enum MenuButtonAction {
    Play,
    Quit,
}

pub(super) fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            _ => NORMAL_BUTTON.into(),
        };
    }
}

pub(super) fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        // if no button has been pressed, do nothing
        if *interaction != Interaction::Pressed {
            return;
        }

        match menu_button_action {
            MenuButtonAction::Quit => app_exit_events.send(AppExit),
            MenuButtonAction::Play => game_state.set(GameState::Game),
        };
    }
}
