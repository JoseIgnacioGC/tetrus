mod blocks;
mod board;

use blocks::Block;
use board::Board;

use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style, terminal, QueueableCommand,
};

pub const COLUMNS: usize = 10;
pub const ROWS: usize = 22;
const BLOCK_FALL_SPEED_MS: u64 = 1000;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut board = Board::new(COLUMNS, ROWS);
    let mut block_fall_start_time = Instant::now();

    print!("\x1B[2J\x1B[H");

    loop {
        if poll(Duration::ZERO)? {
            read()?;
        };

        if !board.is_block_falling {
            let block = Block::get_random();
            if board.try_insert_block(&block).is_err() {
                break;
            };
        }

        if has_past_less_than_x_ms(BLOCK_FALL_SPEED_MS, block_fall_start_time)
            && poll(Duration::from_millis(BLOCK_FALL_SPEED_MS))?
        {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Left | KeyCode::Right => board.move_block_x_axis(event.code),
                    KeyCode::Down => {
                        board.try_move_block_down_or_set().ok();
                    }
                    KeyCode::Char('z') | KeyCode::Char('x') => {
                        board.try_rotate_block().ok();
                    }
                    KeyCode::Char(' ') => while board.try_move_block_down_or_set().is_ok() {},
                    KeyCode::Esc => break,
                    _ => {
                        continue;
                    }
                }
            }
        } else {
            board.try_move_block_down_or_set().ok();
            block_fall_start_time = Instant::now()
        }

        let term_width = terminal::size()?.0.into();
        let formated_board = board.get_formated_board(term_width);

        stdout
            .queue(cursor::MoveTo(ROWS as u16, 0))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(style::Print("\n"))?
            .queue(style::Print(formated_board))?
            .flush()?;
    }
    let term_width = terminal::size()?.0.into();
    println!("\n\n{: ^term_width$}\n", "You lost!!");
    Ok(())
}

fn has_past_less_than_x_ms(x_ms_has_past: u64, start_time: Instant) -> bool {
    start_time.elapsed() < Duration::from_millis(x_ms_has_past)
}
