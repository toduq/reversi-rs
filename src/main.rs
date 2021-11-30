use getopts::Options;

mod board;
mod mobility;
mod search;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("", "ffo", "Solve FFO");
    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!("{}", f.to_string()));
    if matches.opt_present("ffo") {
        solve_ffo();
    } else {
        run_game();
    }
}

fn run_game() {
    let mut b = board::Board::new();
    loop {
        let mob = mobility::get_mobility(&b);
        let opp_mob = mobility::get_mobility(&b.swap());
        if mob > 0 {
            let best_move = search::find_best_move(&b, 1000);
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

/// https://github.com/primenumber/issen-rs/blob/21cae3698e4e79d527e5bb867f7ef111d73b3373/problem/fforum-1-19.obf
fn solve_ffo() {
    #[rustfmt::skip]
    let cases = vec![
        ("--XXXXX--OOOXX-O-OOOXXOX-OXOXOXXOXXXOXXX--XOXOXX-XXXOOO--OOOOO--", 18, 62),
        // ("-XXXXXX---XOOOO--XOXXOOX-OOOOOOOOOOOXXOOOOOXXOOX--XXOO----XXXXX-", 10, 24),
    ];
    for (s, score, idx) in cases {
        let b = board::parse(s);
        println!("{}", b);
        let started = std::time::Instant::now();
        let result = search::complete_search(&b);
        println!("search finished in {}ms", started.elapsed().as_millis());
        assert_eq!(result.score, score);
        assert_eq!(result.idx, idx);
    }
}
