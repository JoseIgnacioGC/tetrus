mod board_widget;
mod metrics_widget;

#[cfg(debug_assertions)]
mod debug_widget;

#[cfg(debug_assertions)]
use crate::tui::debug_widget::DebugWidget;
use crate::{
    blocks,
    tui::{
        board_widget::{
            BoardState::{Brake, Continue, Pass},
            BoardWidget,
        },
        metrics_widget::MetricsWidget,
    },
};
use ratatui::DefaultTerminal;
use std::{io, time::Instant};

const COLUMNS: u16 = 10;
const ROWS: u16 = 22;
pub struct Game {
    time: Instant,

    metrics_widget: MetricsWidget,
    board_widget: BoardWidget,

    #[cfg(debug_assertions)]
    debug_widget: DebugWidget,
}

// TODO: fix fps drop to 52 after widgets refactor
impl Game {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            metrics_widget: MetricsWidget::new(),
            board_widget: BoardWidget::new(),

            #[cfg(debug_assertions)]
            debug_widget: DebugWidget::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match self.board_widget.run()? {
                Brake => break,
                Continue => continue,
                Pass => (),
            };

            self.draw(terminal);
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) {
        use ratatui::{
            macros::{constraint, horizontal, line, vertical},
            style::Stylize,
            widgets::{Block, Paragraph},
        };

        terminal
            .draw(|frame| {
                let [title_area, game_area] = vertical![== 3,== ROWS].areas(frame.area());
                let [left_area, board_area, next_blocks_area] =
                    horizontal![*= 1, == COLUMNS * 2 + 3, *= 1].areas(game_area);
                let [hold_area, metrics_area] = vertical![*= 1, == 8].areas(left_area);

                frame.render_widget(
                    line![
                        "T".red(),
                        "E".fg(blocks::ORANGE),
                        "T".yellow(),
                        "R".green(),
                        "U".cyan(),
                        "S".magenta(),
                    ]
                    .centered(),
                    title_area.centered_vertically(constraint!(== 1)),
                );

                self.metrics_widget
                    .copy_metrics(&self.board_widget.board, &self.time);
                frame.render_widget(&self.metrics_widget, metrics_area);

                #[cfg(debug_assertions)]
                {
                    self.debug_widget.copy_metrics(&self.board_widget.board);
                    frame.render_widget(&mut self.debug_widget, metrics_area);
                }

                frame.render_widget(&mut self.board_widget, board_area);

                frame.render_widget(
                    Paragraph::new("hold")
                        .block(Block::default())
                        .right_aligned(),
                    hold_area,
                );
                frame.render_widget(
                    Paragraph::new("next")
                        .block(Block::default())
                        .left_aligned(),
                    next_blocks_area,
                );
            })
            .expect("Draw error");
    }
}
