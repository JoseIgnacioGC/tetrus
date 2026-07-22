use crate::{blocks, blocks_manager::BlocksManager, board::Board};
use ratatui::DefaultTerminal;
use std::{
    io,
    time::{Duration, Instant},
};

const COLUMNS: u16 = 10;
const ROWS: u16 = 22;
const GOAL_MULTIPLIER: usize = 5;
const MAX_FALL_SPEED: usize = 20;

// TODO: refactor code following this style: https://github.com/ratatui/ratatui/blob/main/examples/apps/colors-rgb/src/main.rs#L69
pub struct Game {
    time: Instant,
    fall_speed: Duration,
    score: usize,
    cleared_lines: usize,
    level: usize,
    fps: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            fall_speed: Duration::ZERO,
            level: 1,
            cleared_lines: 0,
            score: 0,
            fps: 60,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_60fps_interval: Duration = Duration::from_secs_f32(1.0 / 60.0);

        let mut board = Board::new(COLUMNS, ROWS);
        let mut blocks_manager = BlocksManager::new();

        let mut last_tick = Instant::now();
        let mut acc_time = Duration::ZERO;

        let mut last_fps_count = Instant::now();
        let mut fps_counter = 0;

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_tick);
            last_tick = current_time;

            acc_time += delta_time;

            fps_counter += 1;
            if current_time.duration_since(last_fps_count) >= Duration::from_secs(1) {
                self.fps = fps_counter;
                fps_counter = 0;
                last_fps_count = current_time;
            }

            self.update_metrics();

            use crossterm::event::{poll, read, KeyCode};
            while poll(Duration::ZERO)? {
                if let Some(event) = read().map_or(None, |e| e.as_key_press_event()) {
                    match event.code {
                        KeyCode::Left | KeyCode::Right => board.move_block_x_axis(event.code),
                        KeyCode::Down => {
                            let _ = board.move_block_down_or_set();
                        }
                        KeyCode::Char('z') | KeyCode::Char('x') => {
                            let _ = board.rotate_block(event.code);
                        }
                        KeyCode::Char(' ') => while board.move_block_down_or_set() {},
                        KeyCode::Esc => return Ok(()),
                        _ => (),
                    }
                }
            }

            if !board.is_block_falling {
                let block = blocks_manager.get_next_block();
                if !board.insert_block(block) {
                    break;
                };
            }

            while acc_time >= self.fall_speed {
                acc_time -= self.fall_speed;
                let _ = board.move_block_down_or_set();
            }

            self.draw(terminal, &mut board);

            let elapsed = current_time.elapsed();
            if elapsed < tick_60fps_interval {
                std::thread::sleep(tick_60fps_interval - elapsed);
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal, board: &mut Board) {
        use crate::utils::{integer_format::usize_to_superscript, time::format_instant};
        use ratatui::{
            layout::Offset,
            macros::{constraint, horizontal, line, text, vertical},
            style::Stylize,
            widgets::{Block, Paragraph},
        };

        self.update_metrics();

        terminal
            .draw(|frame| {
                let [title_area, game_area] = vertical![== 3,== ROWS].areas(frame.area());
                let [left_area, board_area, next_blocks_area] =
                    horizontal![*= 1, == COLUMNS * 2 + 3, *= 1].areas(game_area);
                let [hold_area, metrics_area] = vertical![== 100%, == 8].areas(left_area);
                self.cleared_lines = board.cleaned_lines;

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
                            usize_to_superscript(self.cleared_lines),
                            usize_to_superscript(self.level * GOAL_MULTIPLIER)
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
            })
            .expect("Draw error");
    }

    fn update_level(&mut self) {
        let curr_goal = self.level * GOAL_MULTIPLIER;
        if self.cleared_lines >= curr_goal {
            self.level += 1;
        }
    }

    fn update_fall_speed(&mut self) {
        if self.level > MAX_FALL_SPEED {
            return;
        }

        self.fall_speed = Duration::from_secs_f32(
            (0.8 - ((self.level as f32 - 1.0) * 0.007)).powf(self.level as f32 - 1.0),
        )
    }

    fn update_metrics(&mut self) {
        self.update_level();
        self.update_fall_speed();
    }
}
