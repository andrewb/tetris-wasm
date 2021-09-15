mod bag;
mod board;
mod game;
mod matrix;
mod piece;

const ROW_COUNT: u8 = 12;
const COL_COUNT: u8 = 12;

fn main() {
    let mut game = game::Game::new(ROW_COUNT, COL_COUNT);
    for _ in 0..8 {
        game.update(1.0);
        println!("{}", game);
    }
}
