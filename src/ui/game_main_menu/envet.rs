use crate::unity::cooldown_time::*;
use bevy::{ecs::error::info, platform::thread, prelude::*};
use std::time::Duration;

use crate::ui::{
    GameMenu,
    game_main_menu::{
        GameStatus,
        bord_game::{
            CreateAddData, GameCreate, GameCreateEnumData, MAX_APPLE, MAX_TABLE, MIN_APPLE,
            MIN_TABLE, TextBox,
        },
        new_game_menu,
    },
};

pub fn new_game(
    game_menu_read: Res<State<GameMenu>>,
    mut game_status: ResMut<NextState<GameStatus>>,
    game_new_read: Res<State<GameStatus>>,
    commands: Commands,
) {
    if *game_menu_read.get() != GameMenu::Game && *game_new_read.get() == GameStatus::None {
        return;
    }
    if *game_new_read.get() != GameStatus::None {
        return;
    }
    info!(
        "Game start {:?} {:?}",
        game_menu_read.get(),
        game_new_read.get()
    );

    new_game_menu(commands);
    game_status.set(GameStatus::NewGame);
}

// i know this code not the bestter but i dont have idea
pub fn update_create_data(
    mut game_create_data: ResMut<GameCreate>,
    interaction_query: Query<(
        &Interaction,
        &GameCreateEnumData,
        &CreateAddData,
        &mut Cooldown,
        Option<&ActiveCooldown>,
        Entity,
    )>,
    text_box_query: Query<(&TextBox, &GameCreateEnumData, &Children)>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
) {
    //info!("{:#?}", text_box_query);
    for (inter, game_enum, add_data, mut cooldown, active, entity) in interaction_query {
        match *inter {
            Interaction::Pressed => {
                if !active.is_none() {
                    return;
                }
                info!("Create");
                cooldown.0.reset();
                commands.entity(entity).insert(ActiveCooldown);

                let table = *game_create_data.tabel();
                let apple = *game_create_data.apple();
                match *game_enum {
                    GameCreateEnumData::TableData => {
                        game_create_data.set_tabel(update_table(table + write_data(add_data)));

                        for (_, game_enum, children) in text_box_query {
                            if *game_enum == GameCreateEnumData::TableData {
                                let mut text = text_query.get_mut(children[0]).unwrap();
                                **text = game_create_data.tabel().to_string();
                            }
                        }
                    }
                    GameCreateEnumData::AppleData => {
                        game_create_data.set_apple(update_apple(apple + write_data(add_data)));

                        for (_, game_enum, children) in text_box_query {
                            if *game_enum == GameCreateEnumData::AppleData {
                                let mut text = text_query.get_mut(children[0]).unwrap();
                                **text = game_create_data.apple().to_string();
                            }
                        }
                    }
                };
            }
            _ => {}
        }
    }
}

fn update_table(data: i32) -> i32 {
    if data > MAX_TABLE {
        error!("U can Max TABLE {}", MAX_TABLE);
        return MAX_TABLE;
    }
    if data < MIN_TABLE {
        error!("U can Min TABLE {}", MIN_TABLE);
        return MIN_TABLE;
    }
    data
}

fn update_apple(data: i32) -> i32 {
    if data > MAX_APPLE {
        error!("U can Max TABLE {}", MAX_APPLE);
        return MAX_APPLE;
    }
    if data < MIN_APPLE {
        error!("U can Min TABLE {}", MIN_APPLE);
        return MIN_APPLE;
    }
    data
}

fn write_data(my_enum: &CreateAddData) -> i32 {
    match my_enum {
        CreateAddData::P5 => 5,
        CreateAddData::P1 => 1,
        CreateAddData::D5 => -5,
        CreateAddData::D1 => -1,
    }
}
