mod board_widget;
mod menu_widget;
mod metrics_widget;

#[cfg(debug_assertions)]
mod debug_widget;

#[cfg(debug_assertions)]
use crate::tui::debug_widget::DebugWidget;
use crate::{
    blocks,
    tui::{
        board_widget::{BoardState, BoardWidget},
        gameover_widget::{GameoverState, GameoverWidget},
        menu_widget::{MenuState, MenuWidget},
        metrics_widget::MetricsWidget,
    },
};
use ratatui::{
    layout::Offset,
    macros::{constraint, horizontal, line, vertical},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

use crossterm::event::{poll, read};
use std::{io, time::Duration};

pub const COLUMNS: u16 = 10;
pub const ROWS: u16 = 22;

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
    gameover_widget: GameoverWidget<'a>,
    #[cfg(debug_assertions)]
    debug_widget: DebugWidget,
}

// TODO: fix fps drop after widgets refactor
// TODO: blocks can not rotate when they are touching one side of the board
impl<'a> Game<'a> {
    pub fn new() -> Self {
        Self {
            title: line![
                "T".red(),
                "E".fg(blocks::ORANGE),
                "T".yellow(),
                "R".green(),
                "U".cyan(),
                "S".magenta(),
            ]
            .centered(),
            game_state: GameState::Menu,

            menu_widget: MenuWidget::new(),
            metrics_widget: MetricsWidget::new(),
            board_widget: BoardWidget::new(),
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
                            self.board_widget.board.new_game();
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
                            self.board_widget.board.new_game();
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
        let [_, area] = vertical![*=1, *= 1].areas(frame.area());

        frame.render_widget(&self.title, area.offset(Offset { x: 0, y: -4 }));

        frame.render_widget(&mut self.menu_widget, area.offset(Offset { x: 0, y: -2 }));
    }

    fn render_game(&mut self, frame: &mut Frame) {
        let [title_area, game_area] = vertical![== 3,== ROWS].areas(frame.area());
        let [left_area, board_area, next_blocks_area] =
            horizontal![*= 1, == COLUMNS * 2 + 3, *= 1].areas(game_area);
        let [hold_area, metrics_area] = vertical![*= 1, == 8].areas(left_area);

        frame.render_widget(
            &self.title,
            title_area.centered_vertically(constraint!(== 1)),
        );

        self.metrics_widget.copy_metrics(&self.board_widget.board);
        frame.render_widget(&self.metrics_widget, metrics_area);

        #[cfg(debug_assertions)]
        {
            self.debug_widget.copy_metrics(&self.board_widget.board);
            frame.render_widget(&mut self.debug_widget, metrics_area);
        }

        frame.render_widget(&self.board_widget, board_area);

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

    fn render_gameover(&mut self, frame: &mut Frame) {
        frame.render_widget(&mut self.gameover_widget, frame.area());
    }
}

mod gameover_widget;
