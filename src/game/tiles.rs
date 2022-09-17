// use std::collections::HashMap;

use terminal::Color;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TileType {
    Normal,
    Middle,
    MultiplyWord(u8),
    MultiplyLetter(u8),
}

#[derive(Debug)]
#[allow(dead_code)]
///Represents a single Tile on the board
pub struct Tile(pub Option<char>, pub TileType);

impl Tile {
    pub fn character_value(&self) -> u8 {
        match self.0 {
            Some(c) => match c {
                'A' | 'E' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
                'D' | 'G' => 2,
                'B' | 'C' | 'M' | 'P' => 3,
                'F' | 'H' | 'V' | 'W' | 'Y' => 4,
                'K' => 5,
                'J' | 'X' => 8,
                'Q' | 'Z' => 10,
                _ => 0,
            },
            None => 0,
        }
    }

    pub fn tile_value(&self) -> u8 {
        self.character_value()
            * match self.1 {
                TileType::MultiplyLetter(n) => n,
                _ => 1,
            }
    }

    pub fn get_color(&self) -> Color {
        match self.1 {
            TileType::Normal => Color::Reset,
            TileType::Middle => Color::White,
            TileType::MultiplyWord(_) => Color::Red,
            TileType::MultiplyLetter(_) => Color::DarkYellow,
        }
    }
}

pub type Board = Vec<Vec<Tile>>;

/*
fn distance_from_middle(n: u8) -> u8 {
    (7 - n as i32).abs() as u8
}
*/

pub fn generate_empty_board() -> Board {
    const N: Tile = Tile(None, TileType::Normal);
    const MID: Tile = Tile(None, TileType::Middle);
    const L2: Tile = Tile(None, TileType::MultiplyLetter(2));
    const L3: Tile = Tile(None, TileType::MultiplyLetter(3));
    const W2: Tile = Tile(None, TileType::MultiplyWord(2));
    const W3: Tile = Tile(None, TileType::MultiplyWord(3));

    let board: Board = vec![
        vec![W3, N, N, L2, N, N, N, W3, N, N, N, L2, N, N, W3],
        vec![N, W2, N, N, N, L3, N, N, N, L3, N, N, N, L2, N],
        vec![N, N, W2, N, N, N, L2, N, L2, N, N, N, L2, N, N],
        vec![L2, N, N, L2, N, N, N, L2, N, N, N, L2, N, N, L2],
        vec![N, N, N, N, L2, N, N, N, N, N, L2, N, N, N, N],
        vec![N, L3, N, N, N, L3, N, N, N, L3, N, N, N, L3, N],
        vec![N, N, L2, N, N, N, L2, N, L2, N, N, N, L2, N, N],
        vec![W3, N, N, L2, N, N, N, MID, N, N, N, L2, N, N, W3],
        vec![N, N, L2, N, N, N, L2, N, L2, N, N, N, L2, N, N],
        vec![N, L3, N, N, N, L3, N, N, N, L3, N, N, N, L3, N],
        vec![N, N, N, N, L2, N, N, N, N, N, L2, N, N, N, N],
        vec![L2, N, N, L2, N, N, N, L2, N, N, N, L2, N, N, L2],
        vec![N, N, W2, N, N, N, L2, N, L2, N, N, N, L2, N, N],
        vec![N, W2, N, N, N, L3, N, N, N, L3, N, N, N, L2, N],
        vec![W3, N, N, L2, N, N, N, W3, N, N, N, L2, N, N, W3],
    ];

    board
}
