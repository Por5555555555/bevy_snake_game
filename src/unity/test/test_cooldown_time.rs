use bevy::{ecs::entity, prelude::*};

use crate::unity::cooldown_time::*;

pub struct TestCoolDown;

#[derive(Component, Debug)]
struct Item;

impl Plugin for TestCoolDown {
    fn build(&self, app: &mut App) {
        info!("add test cool down");
        app.add_systems(Startup, setup);
        app.add_systems(Update, event);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(children![(Item, Cooldown(add_time_onec(3.)))]);
}

fn event(
    mut commands: Commands,
    test_query: Query<(Entity, Option<&ActiveCooldown>, &mut Cooldown), With<Item>>,
) {
    for (entity, act, mut cool) in test_query {
        if !act.is_none() {
            return;
        }
        info!("add new");
        cool.0.reset();
        commands.entity(entity).insert(ActiveCooldown);
    }
}
