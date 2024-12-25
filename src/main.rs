mod blocks;
mod board;

use blocks::Block;
use board::Board;

use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    style, terminal, QueueableCommand,
};

pub const COLUMNS: usize = 10;
pub const ROWS: usize = 22;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut board = Board::new(COLUMNS, ROWS);

    print!("\x1B[2J\x1B[H");

    loop {
        if poll(Duration::from_secs(0))? {
            let _ = read()?;
        };

        if !board.is_block_falling {
            let block = Block::get_random();
            if board.insert_block(&block).is_err() {
                break;
            };
        } else if poll(Duration::from_millis(500))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Left | KeyCode::Right => board.move_block_x_axis(),
                    KeyCode::Down => {
                        let _ = board.move_block_down_or_set().is_err();
                    }
                    KeyCode::Char('z') => println!("{}", event.code),
                    KeyCode::Char('x') => println!("{}", event.code),
                    KeyCode::Char(' ') => loop {
                        if board.move_block_down_or_set().is_err() {
                            break;
                        }
                    },
                    KeyCode::Backspace => println!("Space?: {}", event.code),
                    KeyCode::Esc => break,
                    _ => {
                        // should re-call poll with the remind time
                        let _ = board.move_block_down_or_set().is_err();
                    }
                }
            } else {
                todo!()
            }
            // let _ = board.move_block_down_or_set().is_err();
        } else {
            let _ = board.move_block_down_or_set().is_err();
        }

        let term_width = terminal::size()?.0 as usize;
        let formated_board = board.get_formated_board(term_width);

        stdout
            .queue(cursor::MoveTo(ROWS as u16, 0))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(style::Print("\n"))?
            .queue(style::Print(formated_board))?
            .flush()?;

        poll(Duration::from_millis(0))?;

        // sleep(Duration::from_millis(1000));
    }
    let term_width = terminal::size()?.0 as usize;
    println!("\n\n{: ^term_width$}\n", "You lost!!");
    Ok(())
}