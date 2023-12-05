mod game;

/// Return the game ID if game is possible, 0 otherwise
fn process_game(game: &str) -> u32 {
    let (_, game) = game::Game::parse(game).unwrap();
    if game.possible() {
        game.id()
    } else {
        0
    }
}

fn main() {
    let total: u32 = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(process_game)
        .sum();

    println!("{total}");
}
