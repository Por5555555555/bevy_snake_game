use crate::ui::game_main_menu::{envet::update_create_data, new_game};
use bevy::prelude::*;
use getset::{Getters, Setters};

#[derive(Component, Debug)]
pub struct GameMain;

pub const MAX_TABLE: i32 = 50;
pub const MIN_TABLE: i32 = 10;
pub const MAX_APPLE: i32 = 20;
pub const MIN_APPLE: i32 = 1;

#[derive(Getters, Setters, Resource, Component, Debug)]
pub struct GameCreate {
    #[getset(get = "pub", set = "pub")]
    tabel: i32,
    #[getset(get = "pub", set = "pub")]
    apple: i32,
    #[getset(get = "pub", set = "pub")]
    cooldown: f32,
}

impl Default for GameCreate {
    fn default() -> Self {
        Self {
            tabel: MIN_TABLE,
            apple: MIN_APPLE,
            cooldown: 1.,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameStatus {
    Playing,
    StopGame,
    NewGame,
    EndGame,

    #[default]
    None,
}

// GameCreateEnumData and CreateAddData use for button add Table and apple data
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameCreateEnumData {
    TableData,
    AppleData,
}

// P -> + and D -> -
// Ex. P5 -> +5 and D1 -> -1
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateAddData {
    P5,
    P1,
    D5,
    D1,
}

// Create for filter text update new data
#[derive(Component)]
pub struct TextBox;

impl Plugin for GameMain {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStatus>();
        app.init_resource::<GameCreate>();
        app.add_systems(Update, (new_game, update_create_data).chain());
    }
}
