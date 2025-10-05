// use bevy::{ecs::entity, prelude::*};
//
// pub struct TimeCoolDownTest;
//
// #[derive(Component)]
// struct Cooldown(Timer);
//
// #[derive(Component, Debug)]
// struct ActiveCooldown;
//
// impl Plugin for TimeCoolDownTest {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup);
//         app.add_systems(Update, (load_time, update_time));
//     }
// }
//
// #[derive(Component, Debug)]
// struct TimeItem;
//
// fn setup(mut commands: Commands) {
//     commands.spawn(children![(
//         TimeItem,
//         Cooldown(Timer::from_seconds(10., TimerMode::Once))
//     )]);
// }
//
// fn load_time(
//     mut commands: Commands,
//     time_item_no_cool: Query<
//         (
//             Entity,
//             Option<&ActiveCooldown>,
//             //&mut TimeItem,
//             &mut Cooldown,
//         ),
//         With<TimeItem>,
//     >,
// ) {
//     for (i, j, mut z) in time_item_no_cool {
//         if j.is_none() {
//             info!("new time");
//             z.0.reset();
//             commands.entity(i).insert(ActiveCooldown);
//         } else {
//             return;
//         }
//     }
// }

// fn update_time(
//     time: Res<Time>,
//     mut commands: Commands,
//     time_item: Query<(Entity, &mut Cooldown), With<ActiveCooldown>>,
// ) {
//     //info!("update {:#?}", time_item);
//     for (e, mut t) in time_item {
//         t.0.tick(time.delta());
//         //info!("{:#?}", t.0);
//         if t.0.just_finished() {
//             info!("sucess");
//             commands.entity(e).remove::<ActiveCooldown>();
//         } else {
//             return;
//         }
//     }
// }
