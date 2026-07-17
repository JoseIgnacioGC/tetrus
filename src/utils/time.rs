use std::time::Instant;

pub fn format_instant(elapsed: &Instant) -> String {
    let duration = elapsed.elapsed();
    let total_ms = duration.as_millis();

    let minutes = (total_ms / 60_000) % 60;
    let seconds = (total_ms / 1_000) % 60;
    let milliseconds = total_ms % 1_000;

    format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
}
