mod blocks;
mod blocks_manager;
mod board;
mod tui;
mod utils;

fn main() {
    ratatui::run(|terminal| {
        tui::Game::new()
            .run(terminal)
            .expect("Error at some point, idk.");
    });
}
