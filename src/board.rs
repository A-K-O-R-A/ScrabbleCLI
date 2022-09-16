// use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TileType {
    Normal,
    Middle,
    MultiplyWord(u8),
    MultiplyCharacter(u8),
}

#[derive(Debug)]
#[allow(dead_code)]
///Represents a single Tile on the board
pub struct Tile(pub Option<char>, pub TileType);

impl Tile {
    pub fn calculate_value(&self) -> u8 {
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
}

//  const LETTER_VCOUNTS: HashMap<char, u8> = HashMap::from([('a', 1), ('b', 2), ('c', 3)]);
