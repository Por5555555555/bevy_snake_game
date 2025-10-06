use bevy::{color::palettes::css::*, prelude::*};

use crate::{
    ui::game_main_menu::{
        GameStatus,
        bord_game::{CreateAddData, GameCreateEnumData, MIN_APPLE, MIN_TABLE, TextBox},
    },
    unity::{button_add::ButtonAddOn, node_add::addon_node::AddOnNode, text::TextOut},
};

use crate::unity::cooldown_time::*;

pub fn new_game_menu(mut commands: Commands) {
    info!("call new game menu");
    let main_node = Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let head_node = Node {
        width: percent(90),
        height: percent(90),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let head_text_node = Node {
        margin: UiRect::all(px(100)),
        ..default()
    };

    // let head_box_chose_node = Node {
    //     width: percent(100.),
    //     height: px(50.),
    //     flex_direction: FlexDirection::Row,
    //     align_items: AlignItems::Center,
    //     justify_content: JustifyContent::Center,
    //     margin: UiRect {
    //         left: px(0),
    //         right: px(0),
    //         top: px(5),
    //         bottom: px(5),
    //     },
    //     ..default()
    // };

    let box_start_game = Node {
        width: px(200.),
        height: px(50.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        border: UiRect::all(px(2)),
        margin: UiRect {
            left: px(0),
            right: px(0),
            top: px(20),
            bottom: px(0),
        },
        ..default()
    };

    commands.spawn((
        DespawnOnExit(GameStatus::NewGame),
        main_node,
        children![(
            head_node,
            BackgroundColor(DARK_GRAY.into()),
            children![
                (
                    Text::new("NewGame"),
                    head_text_node,
                    TextFont {
                        font_size: 44.,
                        ..default()
                    }
                ),
                add_config_button("Max Table ", GameCreateEnumData::TableData),
                add_config_button("Max Apple ", GameCreateEnumData::AppleData),
                (
                    BackgroundColor(WHITE.into()),
                    box_start_game,
                    BorderRadius::all(px(10)),
                    BorderColor::all(Color::BLACK),
                    children![(
                        Text::new("Start Game"),
                        TextColor(BLACK.into()),
                        TextFont {
                            font_size: 22.,
                            ..default()
                        }
                    )]
                ),
            ],
        )],
    ));
}

fn add_config_button(text: &str, game_create_enum_data: GameCreateEnumData) -> impl Bundle {
    let head_box_chose_node = Node {
        width: percent(100.),
        height: px(50.),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect {
            left: px(0),
            right: px(0),
            top: px(5),
            bottom: px(5),
        },
        ..default()
    };

    (
        head_box_chose_node.clone(),
        BackgroundColor(GREEN.into()),
        children![
            (TextOut::init(text).size(18.).out()),
            (
                ButtonAddOn::init()
                    .node(AddOnNode::init().mode_button().set_width(65).set_height(50))
                    .out(),
                CreateAddData::D5,
                game_create_enum_data,
                Cooldown(add_time_onec(0.1)),
                children![(TextOut::init("-5").out())]
            ),
            (
                ButtonAddOn::init()
                    .node(AddOnNode::init().mode_button().set_width(65).set_height(50))
                    .out(),
                CreateAddData::D1,
                game_create_enum_data,
                Cooldown(add_time_onec(0.1)),
                children![(TextOut::init("-1").out())]
            ),
            (add_box_data(game_create_enum_data)),
            (
                ButtonAddOn::init()
                    .node(AddOnNode::init().mode_button().set_width(65).set_height(50))
                    .out(),
                CreateAddData::P5,
                game_create_enum_data,
                Cooldown(add_time_onec(0.1)),
                children![(TextOut::init("+5").out())]
            ),
            (
                ButtonAddOn::init()
                    .node(AddOnNode::init().mode_button().set_width(65).set_height(50))
                    .out(),
                CreateAddData::P1,
                game_create_enum_data,
                Cooldown(add_time_onec(0.1)),
                children![(TextOut::init("+1").out())]
            ),
        ],
    )
}

fn box_save_data() -> Node {
    Node {
        width: px(250.),
        height: px(50.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect {
            left: px(10),
            right: px(10),
            top: px(0),
            bottom: px(0),
        },
        ..default()
    }
}

fn add_box_data(game_create_enum_data: GameCreateEnumData) -> impl Bundle {
    let data = match game_create_enum_data {
        GameCreateEnumData::TableData => MIN_TABLE,
        _ => MIN_APPLE,
    };

    (
        children![(Text::new(data.to_string()), TextColor(BLACK.into()))],
        BackgroundColor(WHITE.into()),
        BorderRadius::all(px(10)),
        box_save_data(),
        game_create_enum_data,
        TextBox,
    )
}

// fn button_chose_node() -> Node {
//     Node {
//         width: px(50.),
//         height: px(50.),
//         align_items: AlignItems::Center,
//         justify_content: JustifyContent::Center,
//         margin: UiRect {
//             left: px(5),
//             right: px(5),
//             top: px(0),
//             bottom: px(0),
//         },
//         ..default()
//     }
// }
//
// fn add_button(
//     text: String,
//     create_add_data: CreateAddData,
//     game_create_enum_data: GameCreateEnumData,
// ) -> impl Bundle {
//     (
//         Button,
//         button_chose_node(),
//         BackgroundColor(DARK_BLUE.into()),
//         BorderRadius::all(px(10)),
//         children![Text::new(text)],
//         game_create_enum_data,
//         create_add_data,
//         Cooldown(add_time_onec(0.25)),
//     )
// }
