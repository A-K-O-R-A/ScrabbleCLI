use std::io::{Stdout, Write};
use terminal::{error, Action, Clear, Color, Retrieved, Terminal, Value};

pub mod tiles;

#[allow(dead_code)]
pub struct Game {
    pub board: tiles::Board,
    pub preview: tiles::Board,
    pub terminal: Terminal<Stdout>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: tiles::generate_empty_board(),
            preview: tiles::generate_empty_board(),
            terminal: terminal::stdout(),
        }
    }

    pub fn render(&self) -> Result<String, terminal::error::ErrorKind> {
        let output = String::new();

        self.terminal.act(Action::ClearTerminal(Clear::All))?;

        for row in &self.board {
            for tile in row {
                print!("├");
                tile.render(&self.terminal)?;
                print!("┤");
            }
            self.terminal
                .act(Action::SetBackgroundColor(Color::Reset))?;
            println!("");
        }

        Ok(output)
    }

    pub fn test(&mut self) {
        //self.board = vec![];
    }
}

#[allow(dead_code)]
fn calculate_word(tiles: Vec<tiles::Tile>) -> u16 {
    let mut sum: u16 = 0;
    let mut word_multiplier: u16 = 1;

    for tile in tiles {
        sum += tile.tile_value() as u16;
        match tile.1 {
            tiles::TileType::MultiplyWord(n) => {
                word_multiplier *= n as u16;
            }
            _ => (),
        };
    }

    sum * word_multiplier
}

#[allow(dead_code)]
fn verify_word(_tiles: Vec<tiles::Tile>) -> bool {
    todo!("Verify if a vector of tiles is a valid word");
}
