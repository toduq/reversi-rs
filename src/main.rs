// use rand::seq::SliceRandom;
// use rand::thread_rng;

mod board;
mod mobility;
mod search;

fn main() {
    let mut b = board::Board::new();
    loop {
        let mobilities = mobility::get_mobility(&b);
        if mobilities == 0 {
            break;
        }
        let best_move = search::find_best_move(&b);
        b = mobility::put(&b, best_move);
        println!("{}", mobility::print_movility(&b));
    }
}
