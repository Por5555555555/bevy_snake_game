mod bevy_cooldown;
pub use bevy_cooldown::{ActiveCooldown, Cooldown, PluginTimer, add_time_onec, add_time_rep};

mod test;
pub use test::TestCoolDown;
