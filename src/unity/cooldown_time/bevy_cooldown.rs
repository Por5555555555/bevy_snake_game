// idea time cooldown
// user create time new cooldown
// my fn check cooldown

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Cooldown(pub Timer);

#[derive(Component, Debug)]
pub struct ActiveCooldown;

pub struct PluginTimer;

// pub fn add_active_cooldown(
//     entity: Entity,
//     act_time: Option<&ActiveCooldown>,
//     mut cooldown: Timer,
//     mut commands: Commands,
// ) {
//     if !act_time.is_none() {
//         return;
//     }
//     cooldown.reset();
//     commands.entity(entity).insert(ActiveCooldown);
// }

fn update_time(
    time: Res<Time>,
    mut commands: Commands,
    time_item: Query<(Entity, &mut Cooldown), With<ActiveCooldown>>,
) {
    //info!("{:#?}", time_item);
    for (entity, mut cooldown) in time_item {
        cooldown.0.tick(time.delta());
        if cooldown.0.just_finished() {
            info!("Clean");
            commands.entity(entity).remove::<ActiveCooldown>();
        }
    }
}

impl Plugin for PluginTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_time);
    }
}

pub fn add_time_onec(num: f32) -> Timer {
    Timer::from_seconds(num, TimerMode::Once)
}

pub fn add_time_rep(num: f32) -> Timer {
    Timer::from_seconds(num, TimerMode::Repeating)
}
