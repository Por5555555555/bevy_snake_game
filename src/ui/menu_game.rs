use bevy::{color::palettes::css::*, prelude::*};

use crate::unity::cooldown_time::{ActiveCooldown, Cooldown};

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
        app.add_systems(Startup, app_menu);
        app.add_systems(Update, (button_system, go_back_to_menu));
    }
}

fn app_menu(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(game_menu());
}

fn game_menu() -> impl Bundle {
    info!("init new menu");

    let main_node = Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let hand_node = Node {
        width: px(500),
        height: px(500),
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_node = Node {
        width: px(165),
        height: px(50),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::all(px(10)),
        border: UiRect::all(px(5)),
        ..default()
    };

    (
        main_node,
        //BackgroundColor(DARK_GRAY.into()),
        DespawnOnExit(GameMenu::MenuMain),
        children![(
            hand_node,
            BackgroundColor(DARK_BLUE.into()),
            children![
                (
                    Text::new("Game snake"),
                    TextFont {
                        font_size: 44.,
                        ..default()
                    },
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    }
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(DARK_GRAY.into()),
                    BorderRadius::MAX,
                    BorderColor::all(Color::BLACK),
                    GameMenu::Game,
                    children![(
                        Text::new("Play"),
                        TextFont {
                            font_size: 33.,
                            ..default()
                        },
                        TextShadow::default()
                    )]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(DARK_GRAY.into()),
                    BorderRadius::MAX,
                    BorderColor::all(Color::BLACK),
                    GameMenu::Quit,
                    children![(
                        Text::new("Quit"),
                        TextFont {
                            font_size: 33.,
                            ..default()
                        },
                        TextShadow::default()
                    )]
                )
            ]
        )],
    )
}

pub fn button_system(
    interaction_query: Query<(&Interaction, &GameMenu, &mut BackgroundColor)>,
    mut game_menu: ResMut<NextState<GameMenu>>,
    mut commands: Commands,
) {
    for (interaction, button_status, mut color) in interaction_query {
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
            Interaction::Hovered => {
                *color = GREEN.into();
            }
            Interaction::None => {
                *color = DARK_GRAY.into();
            }
        }
    }
}

pub fn go_back_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_menu_data: ResMut<NextState<GameMenu>>,
    game_menu_read: Res<State<GameMenu>>,
    mut commands: Commands,
) {
    if *game_menu_read.get() == GameMenu::MenuMain {
        return;
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        game_menu_data.set(GameMenu::Game);
        game_menu_data.set(GameMenu::MenuMain);
        commands.spawn(game_menu());
    }
}
