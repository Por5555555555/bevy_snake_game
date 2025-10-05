mod testuit;
mod ui;
mod unity;

use bevy::{prelude::*, window::WindowResolution};

const APP_SIZE: Vec2 = vec2(800., 800.);

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game snake".to_string(),
                resolution: WindowResolution::new(APP_SIZE.x as u32, APP_SIZE.y as u32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((ui::GameMenuStruct, ui::GameMain))
        //.add_plugins(unity::TestCoolDown)
        .add_plugins(unity::PluginTimer)
        .run();
}
