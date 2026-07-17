use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::Offset,
    macros::{constraint, horizontal, line, text, vertical},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{
    blocks,
    blocks_manager::BlocksManager,
    board::Board,
    utils::{integer_format::usize_to_superscript, time::format_instant},
};

const COLUMNS: u16 = 10;
const ROWS: u16 = 22;

pub struct Game {
    title: Line<'static>,

    time: Instant,
    fall_speed: Duration,
    score: usize,
    lines: usize,
    level: usize,
    fps: usize,
}

impl Game {
    pub fn new() -> Self {
        let title = line![
            "T".red(),
            "E".fg(blocks::ORANGE),
            "T".yellow(),
            "R".green(),
            "U".cyan(),
            "S".magenta(),
        ]
        .centered();

        Self {
            title,
            time: Instant::now(),
            fall_speed: Duration::ZERO,
            level: 5,
            lines: 0,
            score: 0,
            fps: 60,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut board = Board::new(COLUMNS, ROWS);
        let mut blocks_manager = BlocksManager::new();
        let mut block_fall_start_time = self.time;

        loop {
            self.update_fall_speed();

            if !board.is_block_falling {
                let block = blocks_manager.get_next_block();
                if !board.insert_block(block) {
                    break;
                };
                block_fall_start_time = Instant::now();
                terminal.draw(|frame| self.render(frame, &mut board))?;
                continue;
            }

            if block_fall_start_time.elapsed() < self.fall_speed && event::poll(self.fall_speed)? {
                if let Some(event) = event::read().map_or(None, |e| e.as_key_press_event()) {
                    match event.code {
                        KeyCode::Left | KeyCode::Right => board.move_block_x_axis(event.code),
                        KeyCode::Down => {
                            let _ = board.move_block_down_or_set();
                        }
                        KeyCode::Char('z') | KeyCode::Char('x') => {
                            let _ = board.rotate_block(event.code);
                        }
                        KeyCode::Char(' ') => while board.move_block_down_or_set() {},
                        KeyCode::Esc => break,
                        _ => continue,
                    }
                }
            } else {
                let _ = board.move_block_down_or_set();
                block_fall_start_time = Instant::now();
            };

            terminal.draw(|frame| self.render(frame, &mut board))?;
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, board: &mut Board) {
        let [title_area, game_area] = vertical![== 3,== ROWS].areas(frame.area());
        let [left_area, board_area, next_blocks_area] =
            horizontal![*= 1, == COLUMNS * 2 + 3, *= 1].areas(game_area);
        let [hold_area, metrics_area] = vertical![== 100%, == 8].areas(left_area);

        frame.render_widget(
            &self.title,
            title_area.centered_vertically(constraint!(== 1)),
        );

        frame.render_widget(board, board_area);
        frame.render_widget(
            Paragraph::new(text![
                "lv\n",
                usize_to_superscript(self.level),
                "score\n",
                usize_to_superscript(self.score),
                "lines\n",
                format!(
                    "{}⁄{}",
                    usize_to_superscript(self.lines),
                    usize_to_superscript(self.level * 5)
                ),
                "time\n",
                format_instant(&self.time),
            ])
            .right_aligned(),
            metrics_area,
        );

        #[cfg(debug_assertions)]
        {
            frame.render_widget(
                text![
                    "[debug]\n",
                    format!("fps: {}\n", self.fps),
                    format!("fall_speed: {}", self.fall_speed.as_secs_f32()),
                ]
                .left_aligned(),
                metrics_area.offset(Offset::new(3, 0)),
            );
        }

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
    }

    fn update_fall_speed(&mut self) {
        const MAX_FALL_SPEED: usize = 20;
        if self.level > MAX_FALL_SPEED {
            return;
        }

        self.fall_speed = Duration::from_secs_f32(
            (0.8 - ((self.level as f32 - 1.0) * 0.007)).powf(self.level as f32 - 1.0),
        )
    }
}
