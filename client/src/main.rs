use bevy::prelude::*;

/// Глобальное состояние игры
#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
enum AppState {
    /// Первичная загрузка игры (еще до показа основного игрового меню)
    #[default]
    InitialLoading,

    /// Основное игровое меню
    MainMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .run();
}
