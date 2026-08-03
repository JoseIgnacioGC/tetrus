use std::time::Duration;

pub const COLUMNS: u16 = 10;
pub const ROWS: u16 = 22;
pub const GOAL_MULTIPLIER: usize = 5;
pub const MAX_FALL_SPEED_LEVEL: usize = 20;
pub const LOCK_DELAY_FRAMES_DURATION: Duration = Duration::from_millis(500);
pub const MAX_DELAY_FRAMES_LOCK_RESETS: usize = 15;
pub const COMBO_NOTIFICATION_DURATION: Duration = Duration::from_millis(3000);
pub const COMBO_NOTIFICATION_FADE_DELAY: Duration = Duration::from_millis(500);
