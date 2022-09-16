mod board;

#[allow(dead_code)]
pub struct Game {
    pub board: Vec<Vec<board::Tile>>,

}

fn main() {
    let field = board::TileType::MultiplyWord(12);

    println!("{:?}", field);
}

#[allow(dead_code)]
fn calculate_word(tiles: Vec<board::Tile>) -> u16 {
    let mut sum: u16 = 0;
    let mut word_multiplier: u16 = 1;

    for tile in tiles {
        sum += tile.calculate_value() as u16;
        match tile.1 {
            board::TileType::MultiplyWord(n) => {
                word_multiplier *= n as u16;
            }
            _ => (),
        };
    }

    sum * word_multiplier
}
#[allow(dead_code)]
fn verify_word(_tiles: Vec<board::Tile>) -> bool {
    todo!("Verify if a vector of tiles is a valid word");
}
