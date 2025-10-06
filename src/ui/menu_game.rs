use bevy::{color::palettes::css::*, prelude::*};

use crate::unity::{
    button_add::{ButtonStyle, button_add::ButtonAddOn},
    node_add::addon_node::AddOnNode,
    text::TextOut,
};

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Hash, States, Debug)]
pub enum GameMenu {
    #[default]
    MenuMain,
    Game,
    Quit,
}

pub struct GameMenuStruct;

impl Plugin for GameMenuStruct {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenu>();
        app.add_systems(Startup, |mut c: Commands| {
            c.spawn(Camera2d);
        });
        app.add_systems(Startup, app_menu);
        app.add_systems(Update, (button_system, go_back_to_menu));
    }
}

fn app_menu(mut commands: Commands) {
    let main_node = Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let head_node = Node {
        width: px(500),
        height: px(500),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    };

    commands.spawn((
        main_node,
        DespawnOnExit(GameMenu::MenuMain),
        children![(
            head_node,
            BackgroundColor(DARK_BLUE.into()),
            children![
                (
                    TextOut::init("Game snake").set_color(WHITE).size(44.).out(),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    }
                ),
                (
                    ButtonAddOn::init()
                        .set_bg_color(DARK_BLUE)
                        .node(AddOnNode::init().mode_button())
                        .out(),
                    GameMenu::Game,
                    children![(TextOut::init("Play").out())]
                ),
                (
                    ButtonAddOn::init()
                        .set_bg_color(DARK_BLUE)
                        .node(AddOnNode::init().mode_button())
                        .out(),
                    GameMenu::Quit,
                    children![(TextOut::init("Quit").out())]
                )
            ],
        )],
    ));
    //commands.spawn(game_menu());
}

pub fn button_system(
    interaction_query: Query<(&Interaction, &GameMenu, &mut BackgroundColor)>,
    mut game_menu: ResMut<NextState<GameMenu>>,
    mut commands: Commands,
) {
    for (interaction, button_status, mut color) in interaction_query {
        ButtonStyle::init().add_event(*interaction, color);

        match *interaction {
            Interaction::Pressed => {
                match *button_status {
                    GameMenu::Game => {
                        info!("Play game");
                        game_menu.set(GameMenu::Game);
                    }
                    GameMenu::Quit => {
                        info!("Quit game");
                        commands.write_message(AppExit::Success);
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}

pub fn go_back_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_menu_data: ResMut<NextState<GameMenu>>,
    game_menu_read: Res<State<GameMenu>>,
    commands: Commands,
) {
    if *game_menu_read.get() == GameMenu::MenuMain {
        return;
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        game_menu_data.set(GameMenu::Game);
        game_menu_data.set(GameMenu::MenuMain);
        app_menu(commands);
    }
}
