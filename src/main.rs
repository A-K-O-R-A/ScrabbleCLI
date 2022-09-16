mod game;

fn main() {
    let field = tiles::Tile(Some('Z'), tiles::TileType::MultiplyCharacter(3));

    println!("Z * 3 has the value: {:?}", field.tile_value());
}
