mod board_widget;
mod held_block_widget;
mod menu_widget;
mod metrics_widget;
mod movement_widget;
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
    movement_widget::MovementWidget,
    next_blocks_widget::NextBlocksWidget,
};
use ratatui::{
    macros::{constraint, horizontal, line, vertical},
    style::Stylize,
    text::Line,
    DefaultTerminal, Frame,
};

use crossterm::event::{poll, read};
use std::{io, time::Duration};

use crate::{
    colors::ORANGE,
    constants::{COLUMNS, ROWS},
};

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Game,
    GameOver,
}

#[derive(Clone, Copy)]
pub enum ActiveGameMode {
    Endless,
    LearnMoves {
        grid: crate::board::Grid,
        starting_pieces: &'static [crate::blocks::Block],
        gravity: usize,
    },
}

pub struct Game<'a> {
    title: Line<'a>,
    game_state: GameState,
    active_game_mode: ActiveGameMode,

    // TODO: use a state machine to not have every widget in memory at any time
    menu_widget: MenuWidget<'a>,
    metrics_widget: MetricsWidget,
    movement_widget: MovementWidget,
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
            active_game_mode: ActiveGameMode::Endless,

            menu_widget: MenuWidget::new(title),
            metrics_widget: MetricsWidget::new(),
            movement_widget: MovementWidget::new(),
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
                            self.active_game_mode = ActiveGameMode::Endless;
                            self.game_state = GameState::Game;
                            self.board_widget.new_game();
                        }
                        MenuState::EnterGameWithPreset(grid, pieces, gravity) => {
                            self.active_game_mode = ActiveGameMode::LearnMoves {
                                grid,
                                starting_pieces: pieces,
                                gravity,
                            };
                            self.game_state = GameState::Game;
                            self.board_widget
                                .new_game_with_preset(grid, pieces, gravity);
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
                            match self.active_game_mode {
                                ActiveGameMode::Endless => {
                                    self.board_widget.new_game();
                                }
                                ActiveGameMode::LearnMoves {
                                    grid,
                                    starting_pieces,
                                    gravity,
                                } => {
                                    self.board_widget.new_game_with_preset(
                                        grid,
                                        starting_pieces,
                                        gravity,
                                    );
                                }
                            }
                        }
                        GameoverState::EnterMenu => {
                            self.game_state = GameState::Menu;
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
                        match self.active_game_mode {
                            ActiveGameMode::Endless => {
                                let score = self.board_widget.board.stats.score;
                                let lines = self.board_widget.board.stats.cleaned_lines;
                                let level = self.board_widget.board.stats.level;
                                self.gameover_widget
                                    .setup_endless(score, lines, level);
                            }
                            ActiveGameMode::LearnMoves { .. } => {
                                self.gameover_widget.setup_learn_moves();
                            }
                        }
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
        let [_, menu_area, bottom_area] = vertical![*=1, == ROWS, *=1].areas(frame.area());
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

        self.movement_widget.copy_metrics(&self.board_widget.board);
        self.movement_widget.render(movement_area, frame);

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
