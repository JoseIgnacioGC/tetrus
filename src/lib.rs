pub mod blocks;
pub mod blocks_manager;
pub mod board;
pub mod colors;
pub mod constants;
pub mod tui;
pub mod utils;

pub fn run() {
    ratatui::run(|terminal| {
        tui::Game::new()
            .run(terminal)
            .expect("Error at some point, idk.");
    });
}
