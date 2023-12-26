use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::DARK_GRAY;
const BACKGROUND: ClearColor = ClearColor(BACKGROUND_COLOR);

pub(super) struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BACKGROUND);
    }
}
