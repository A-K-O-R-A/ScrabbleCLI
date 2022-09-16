pub mod tiles;

#[allow(dead_code)]
pub struct Game {
    pub board: tiles::Board,
    pub preview: tiles::Board,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: tiles::generate_empty_board(),
            preview: tiles::generate_empty_board(),
        }
    }

    pub fn render(&self) -> String {
        let output = String::new();

        for row in &self.board {
            for tile in row {
                let tile_char = match tile.1 {
                    tiles::TileType::Normal => String::from(" "),
                    tiles::TileType::Middle => String::from("+"),
                    tiles::TileType::MultiplyWord(n) => n.to_string(),
                    tiles::TileType::MultiplyLetter(n) => n.to_string(),
                };
                print!("{}  ", tile_char);
            }
            println!("");
        }

        output
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
