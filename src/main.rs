mod blocks;
mod blocks_manager;
mod board;
mod tui;

fn main() {
    ratatui::run(|terminal| {
        tui::App::new()
            .run(terminal)
            .expect("Error at some point, idk.");
    });
}
