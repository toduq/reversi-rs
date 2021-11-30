// use rand::seq::SliceRandom;
// use rand::thread_rng;

mod board;
mod mobility;
mod search;

fn main() {
    let mut b = board::Board::new();
    loop {
        let mob = mobility::get_mobility(&b);
        let opp_mob = mobility::get_mobility(&b.swap());
        if mob > 0 {
            let best_move = search::find_best_move(&b, 100);
            b = mobility::put(&b, best_move);
        } else if opp_mob > 0 {
            println!("pass");
            b = b.swap();
        } else {
            break;
        }
        println!("{}\n", b);
    }
}
