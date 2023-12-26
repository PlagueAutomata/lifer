use bevy::prelude::*;

#[derive(States, Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Shows splash screen
    ///
    /// Now skips [`GameState::Loading`] and run [`GameState::MainMenu`]
    #[default]
    Splash,

    /// Shows loading process
    Loading,

    /// Starting game, goto settings, quit from game
    MainMenu,

    /// State during gameplay
    Playing,
}
