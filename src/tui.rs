mod board_widget;
mod held_block_widget;
mod menu_widget;
mod metrics_widget;
mod next_blocks_widget;

#[cfg(debug_assertions)]
mod debug_widget;

#[cfg(debug_assertions)]
use crate::tui::debug_widget::DebugWidget;
use crate::tui::{
    board_widget::{BoardState, BoardWidget},
    gameover_widget::{GameoverState, GameoverWidget},
    held_block_widget::HeldBlockWidget,
    menu_widget::{MenuState, MenuWidget},
    metrics_widget::MetricsWidget,
    next_blocks_widget::NextBlocksWidget,
};
use ratatui::{
    macros::{constraint, horizontal, line, vertical},
    style::{Color, Stylize},
    text::{Line, Span},
    DefaultTerminal, Frame,
};
use tachyonfx::{fx, EffectRenderer, Interpolation};

use crossterm::event::{poll, read};
use std::{io, time::Duration};

use crate::{
    colors::{GOLD, ORANGE},
    constants::{COLUMNS, ROWS},
};

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Game,
    GameOver,
}

pub struct Game<'a> {
    title: Line<'a>,
    game_state: GameState,

    menu_widget: MenuWidget<'a>,
    metrics_widget: MetricsWidget,
    board_widget: BoardWidget,
    held_block_widget: HeldBlockWidget,
    next_blocks_widget: NextBlocksWidget,
    gameover_widget: GameoverWidget<'a>,
    #[cfg(debug_assertions)]
    debug_widget: DebugWidget,
}

// TODO: fix fps drop after widgets refactor
impl<'a> Game<'a> {
    pub fn new() -> Self {
        let title = line![
            "T".red(),
            "E".fg(ORANGE),
            "T".yellow(),
            "R".green(),
            "U".cyan(),
            "S".magenta(),
        ]
        .centered();

        Self {
            title: title.clone(),
            game_state: GameState::Menu,

            menu_widget: MenuWidget::new(title),
            metrics_widget: MetricsWidget::new(),
            board_widget: BoardWidget::new(),
            held_block_widget: HeldBlockWidget::new(),
            next_blocks_widget: NextBlocksWidget::new(),
            gameover_widget: GameoverWidget::new(),
            #[cfg(debug_assertions)]
            debug_widget: DebugWidget::new(),
        }
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        while poll(Duration::ZERO)? {
            if let Some(event) = read().map_or(None, |e| e.as_key_press_event()) {
                match self.game_state {
                    GameState::Menu => match self.menu_widget.handle_key_event(event) {
                        MenuState::Brake => return Ok(true),
                        MenuState::EnterGame => {
                            self.game_state = GameState::Game;
                            self.board_widget.new_game();
                        }
                        MenuState::Pass => (),
                    },
                    GameState::Game => match self.board_widget.handle_key_event(event) {
                        BoardState::Brake => return Ok(true),
                        _ => (),
                    },
                    GameState::GameOver => match self.gameover_widget.handle_key_event(event) {
                        GameoverState::Brake => return Ok(true),
                        GameoverState::EnterGame => {
                            self.game_state = GameState::Game;
                            self.board_widget.new_game();
                        }
                        GameoverState::Pass => (),
                    },
                }
            }
        }
        Ok(false)
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            if self.handle_events()? {
                break;
            }

            match self.game_state {
                GameState::Game => {
                    if self.board_widget.update() == BoardState::GameOver {
                        self.game_state = GameState::GameOver;
                    }
                }
                _ => std::thread::sleep(Duration::from_millis(16)),
            }

            terminal.draw(|frame| {
                match self.game_state {
                    GameState::Menu => self.render_menu(frame),
                    GameState::Game => self.render_game(frame),
                    GameState::GameOver => {
                        self.render_game(frame);
                        self.render_gameover(frame);
                    }
                };
            })?;
        }

        ratatui::restore();
        Ok(())
    }

    fn render_menu(&mut self, frame: &mut Frame) {
        let [_, menu_area, bottom_area] = vertical![*=1, == 4, *=1].areas(frame.area());
        let [_, controls_area, _] = vertical![*=1, == 1, == 2].areas(bottom_area);

        frame.render_widget(&mut self.menu_widget, menu_area);

        let controls_hint = line![
            "Use ",
            "[←↓→]".cyan(),
            " move ".dim(),
            "[z][x]".cyan(),
            " rotate ".dim(),
            "[c]".cyan(),
            " hold ".dim(),
            "[Space]".cyan(),
            " drop ".dim(),
            "[p]".cyan(),
            " pause ".dim(),
            "[Esc]".cyan(),
            " quit".dim(),
        ]
        .centered();

        frame.render_widget(controls_hint, controls_area);
    }

    fn render_game(&mut self, frame: &mut Frame) {
        let [_, title_area, game_area, _] =
            vertical![*= 1, == 3, == ROWS, *= 1].areas(frame.area());
        let [left_area, board_area, next_blocks_area] =
            horizontal![*= 1, == COLUMNS * 2 + 3, *= 1].areas(game_area);
        let [hold_area, movement_area, metrics_area] = vertical![*= 1, *= 1, == 8].areas(left_area);

        frame.render_widget(
            &self.title,
            title_area.centered_vertically(constraint!(== 1)),
        );

        let [movement_title_area, combo_area] =
            vertical![== 1, == 1].areas(movement_area.centered_vertically(constraint!(== 2)));

        if let Some((movement, elapsed)) = self.board_widget.board.last_movement() {
            let movement_text = if let Some(rest) = movement.strip_prefix("B2B ") {
                let rest_span = if rest.contains("T-Spin") {
                    Span::from(rest.to_string()).magenta().bold()
                } else {
                    Span::from(rest.to_string()).white().bold()
                };
                Line::from(vec!["B2B ".fg(GOLD).bold(), rest_span]).right_aligned()
            } else if movement.contains("T-Spin") {
                Line::from(movement).magenta().bold().right_aligned()
            } else {
                Line::from(movement).white().bold().right_aligned()
            };
            frame.render_widget(movement_text, movement_title_area);

            let delay = Duration::from_millis(500);
            if elapsed > delay {
                let effect_elapsed = elapsed - delay;
                let mut effect =
                    fx::fade_to_fg(Color::Rgb(50, 50, 50), (1000, Interpolation::CubicOut));
                frame.render_effect(&mut effect, movement_title_area, effect_elapsed.into());
            }
        }

        if let Some((combo_count, elapsed)) = self.board_widget.board.current_combo() {
            let combo_text = Line::from(format!("{} Combo", combo_count))
                .white()
                .bold()
                .right_aligned();
            frame.render_widget(combo_text, combo_area);

            let delay = Duration::from_millis(500);
            if elapsed > delay {
                let effect_elapsed = elapsed - delay;
                let mut effect =
                    fx::fade_to_fg(Color::Rgb(50, 50, 50), (1000, Interpolation::CubicOut));
                frame.render_effect(&mut effect, combo_area, effect_elapsed.into());
            }
        }

        self.metrics_widget.copy_metrics(&self.board_widget.board);
        frame.render_widget(&self.metrics_widget, metrics_area);

        #[cfg(debug_assertions)]
        {
            self.debug_widget.copy_metrics(&self.board_widget.board);
            frame.render_widget(&mut self.debug_widget, metrics_area);
        }

        frame.render_widget(&self.board_widget, board_area);

        self.held_block_widget
            .copy_metrics(&self.board_widget.board);
        frame.render_widget(&self.held_block_widget, hold_area);

        self.next_blocks_widget
            .copy_metrics(&self.board_widget.blocks_manager);
        frame.render_widget(&self.next_blocks_widget, next_blocks_area);
    }

    fn render_gameover(&mut self, frame: &mut Frame) {
        frame.render_widget(&mut self.gameover_widget, frame.area());
    }
}

mod gameover_widget;
