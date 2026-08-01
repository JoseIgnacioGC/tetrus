use std::time::Duration;

pub const COLUMNS: u16 = 10;
pub const ROWS: u16 = 22;
pub const GOAL_MULTIPLIER: usize = 5;
pub const MAX_FALL_SPEED_LEVEL: usize = 20;
pub const LOCK_DELAY_DURATION: Duration = Duration::from_millis(500);
pub const MAX_LOCK_RESETS: usize = 15;
