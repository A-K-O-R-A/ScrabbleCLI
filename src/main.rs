mod game;

fn main() {
    let mut game = game::Game::new();
    game.test();
    let field = game::tiles::Tile(Some('Z'), game::tiles::TileType::MultiplyLetter(3));

    println!("Z * 3 has the value: {:?}", field.tile_value());
}
