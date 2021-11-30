use getopts::Options;

mod board;
mod mobility;
mod search;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("", "solve-ffo", "Solve FFO");
    opts.optopt("", "ffo-start", "FFO start #", "NUMBER");
    opts.optopt("", "ffo-end", "FFO end #", "NUMBER");
    opts.optopt("b", "board", "Start board", "BOARD_EXPRESSION");
    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!("{}", f.to_string()));
    if matches.opt_present("solve-ffo") {
        solve_ffo(
            matches
                .opt_str("ffo-start")
                .unwrap_or_else(|| "1".to_owned())
                .parse()
                .unwrap(),
            matches
                .opt_str("ffo-end")
                .unwrap_or_else(|| "100".to_owned())
                .parse()
                .unwrap(),
        );
    } else {
        run_game(matches.opt_str("board"));
    }
}

fn run_game(b_str: Option<String>) {
    let mut b = b_str
        .map(|s| board::parse(&s))
        .unwrap_or_else(board::Board::new);
    println!("{}\n", b);
    loop {
        let mob = mobility::get_mobility(&b);
        let opp_mob = mobility::get_mobility(&b.pass());
        if mob > 0 {
            let best_move = search::find_best_move(&b, 30000);
            b = mobility::put(&b, best_move);
        } else if opp_mob > 0 {
            println!("pass");
            b = b.pass();
        } else {
            break;
        }
        println!("{}\n", b);
    }
}

/// Benchmark by FFO
///
/// https://github.com/abulmo/edax-reversi/tree/master/problem
/// https://github.com/primenumber/issen-rs/blob/a77b757662630b0dfe2573fe5ac084659cbb9781/src/main.rs
fn solve_ffo(start: usize, end: usize) {
    let mut results = vec![];
    let cases: Vec<(usize, &str)> = FFO_CASES.trim().split('\n').enumerate().collect();
    for (problem, line) in &cases[(start - 1)..=(end - 1)] {
        println!("\n\nFFO#{} {:?}", problem + 1, line);
        let elems: Vec<&str> = line.split(' ').collect();
        let mut b = board::parse(elems[0]);
        if elems[1] == "O;" {
            b = b.pass();
        }
        let expected: Vec<&str> = elems[2].trim_end_matches(';').split(':').collect();
        let expected_idx =
            (expected[0].as_bytes()[1] - b'1') * 8 + (expected[0].as_bytes()[0] - b'A');
        let expected_score: i8 = expected[1].parse().unwrap();
        println!("{}", b);
        let started = std::time::Instant::now();
        let result = search::complete_search(&b);
        println!("search finished in {}ms", started.elapsed().as_millis());
        let passed = result.score == expected_score;
        if !passed {
            println!(
                "[FAILED] expected:{}@{}, actual:{}@{}",
                expected_score, expected_idx, result.score, result.idx
            );
        }
        results.push((
            problem + 1,
            (b.me | b.opp).count_zeros(),
            if passed { "ok" } else { "fail" },
            result.score,
            result.idx,
            expected_score,
            expected_idx,
            result.searched as f64 / 1_000_000.0,
            started.elapsed().as_millis() as f64 / 1000.0,
            result.searched as f64 / (started.elapsed().as_micros() + 1) as f64,
        ));
    }

    println!("| No | empties | passed | result | answer |  nodes |   time |      NPS |");
    println!("| --:| ------: | -----: | -----: | -----: | -----: | -----: | -------: |");
    for r in results {
        println!(
            "| {:2} | {:7} | {:>6} | {:+3}@{:2} | {:+3}@{:2} | {:>5.1}M | {:5.1}s | {:5.1}M/s |",
            r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.8
        );
    }
}

// https://github.com/abulmo/edax-reversi/tree/master/problem
const FFO_CASES: &str = r"
--XXXXX--OOOXX-O-OOOXXOX-OXOXOXXOXXXOXXX--XOXOXX-XXXOOO--OOOOO-- X; G8:+18; H1:+12; H7:+6; A2:+6; A3:+4; B1:-4; A4:-22; G2:-24;
-XXXXXX---XOOOO--XOXXOOX-OOOOOOOOOOOXXOOOOOXXOOX--XXOO----XXXXX- X; A4:+10; B2:+0; A3:-6; G7:-8; A7:-12; H7:-14; B7:-14; H2:-24;
----OX----OOXX---OOOXX-XOOXXOOOOOXXOXXOOOXXXOOOOOXXXXOXO--OOOOOX X; D1:+2; G3:+0; B8:-2; B1:-4; C1:-4; A2:-4; A3:-6; B2:-12;
-XXXXXX-X-XXXOO-XOXXXOOXXXOXOOOX-OXOOXXX--OOOXXX--OOXX----XOXXO- X; H8:+0; A5:+0; B6:-4; B7:-4; A6:-8; B2:-12; H2:-26;
-OOOOO----OXXO-XXXOXOXX-XXOXOXXOXXOOXOOOXXXXOO-OX-XOOO---XXXXX-- X; G8:+32; G2:+12; B2:-20; G6:-26; G1:-32; G7:-34;
--OXXX--OOOXXX--OOOXOXO-OOXOOOX-OOXXXXXXXOOXXOX--OOOOX---XXXXXX- X; A1:+14; H3:+14; A8:+12; H2:+8; G2:+8; H4:+4; G7:+4; A7:-22; B1:-24;
--OXXO--XOXXXX--XOOOXXXXXOOXXXXXXOOOOXXX-XXXXXXX--XXOOO----XXOO- X; A6:+8; G1:+0; A1:-2; H8:-6; H7:-14; B1:-30;
---X-X--X-XXXX--XXXXOXXXXXXOOOOOXXOXXXO-XOXXXXO-XOOXXX--XOOXXO-- O; E1:+8; H2:+4; G2:+4; B2:+4; G7:+4; B1:+2; G1:-6; C1:-8;
--XOXX--O-OOXXXX-OOOXXXX-XOXXXOXXXOXOOOXOXXOXOXX--OXOO----OOOO-- O; G7:-8; A4:-8; B1:-16; A7:-16; B7:-26; A3:-30; G1:-38; H7:-40;
-XXXX-----OXXX--XOXOXOXXOXOXXOXXOXXOXOOOXXXOXOOX--OXXO---OOOOO-- O; B2:+10; B7:+4; F1:+0; A7:-4; A2:-6; G2:-12; H2:-16; H7:-20;
---O-XOX----XXOX---XXOOXO-XXOXOXXXXOOXOX-XOOXXXXXOOOXX-XOOOOOOO- O; B3:+30; C2:+26; A6:+24; G7:+20; C3:+18; D2:+16; B4:+10; E1:+6;
--O--O--X-OOOOX-XXOOOXOOXXOXOXOOXXOXXOOOXXXXOOOO--OXXX---XXXXX-- O; B7:-8; A7:-10; G7:-14; G8:-14; H2:-16; G1:-16; H1:-20;
--XXXXX--OOOXX---OOOXXXX-OXOXOXXOXXXOXXX--XOXOXX--OXOOO--OOOOO-- X; B7:+14; A4:+0; A3:-8; B1:-18; G8:-20; H7:-20; A2:-24;
--XXXXX---OOOX---XOOXXXX-OOOOOOOOOOXXXOOOOOXXOOX--XXOO----XXXXX- X; A3:+18; A4:+12; B1:+8; G7:-4; H7:-14; A7:-24; B7:-24; B2:-28;
----O------OOX---OOOXX-XOOOXOOOOOXXOXXOOOXXXOOOOOXXXOOXO--OOOOOX X; G3:+4; B8:+4; F1:+0; C1:+0; C2:-2; D1:-4; B2:-8; A3:-8;
-XXXXXX-X-XXXOO-XOXXXOOXXOOXXXOX-OOOXXXX--OOXXXX---OOO----XOX-O- X; F8:+24; C7:+20; A5:+6; H1:+6; B6:+0; B7:-2; A6:-6; H2:-26;
-OOOOO----OXXO-XXXOOOXX-XXOXOXXOXXOOXOOOXXXXOO-OX-XOO----XXXX--- X; F8:+8; G2:+6; G6:-24; G1:-32; F7:-32; G7:-34; B2:-38;
-XXX------OOOX--XOOOOOXXOXOXOOXXOXXOOOOOXXXOXOOX--OXXO---OOOOO-- X; G2:-2; B7:-6; F1:-8; E1:-10; H7:-12; G8:-14; G7:-14; A2:-18; B2:-18;
--OXXO--XOXXXX--XOOOOXXXXOOOXXXXX-OOOXXX--OOOOXX--XXOOO----XXOO- X; B6:+8; H8:+4; B7:+0; G1:-6; B5:-16; H7:-16; B1:-24;
XXXOXXXXOXXXXXXXOOXXXXXXOOOXXXXXOOOXXOO-OOOOO---OOOOOOO-OOOOOOO- X; H5:+6; G6:-2; F6:-4; H6:-10;
OOOOOOOOXOOXXX--XXOOXOO-XOXOOO--XOOOOX--XOOXOO--XOOOOO--XXXX---- O; G5:+0; G2:-2; G4:-4; G6:-6;
--OOOO--X-OOOOO-XXOOXOXXXOXOXXXXXXXOXXXX-XXOXOXX--OXXX-X----X--- O; G8:+2; A6:+0; F8:-4; A7:-4; H2:-4; B2:-6; D8:-8; B7:-14; G7:-26;
--O-------OOX---OOOXXXO-OOOOXOXXXXXOOXOXXXXXXOOXX-XXXXOX--XXXX-- X; A2:+4; D1:-20; H3:-20; B1:-30; G2:-30; E1:-30; F2:-34; G8:-34; B2:-36; H2:-38;
--O--O-----OOOX--X-XOXOO--XXXOOOXXXXOOOOXXXOXXOOXXXXXX--XOXX-O-- O; C3:+0; B4:-4; C2:-8; E8:-12; G7:-14; H2:-16; G1:-24;
----X------XXXO--OOOXXXXXOOOOXXO-XXOOXXOOOXOXXXXOOOXX---X-XXXX-- O; G1:+0; A5:+0; F1:-4; D1:-6; F7:-8; C2:-10; G7:-10; H2:-12; H7:-16;
-OOOOO----OXXO---OOOOXXO-OOOXOXX-OOXOOXX-XOXXOXX--O-XXXX--O----O X; D8:+0; A6:-2; A4:-6; B7:-6; A5:-12; G1:-16; A2:-16; A3:-18; H2:-18; B8:-20; G2:-20; B2:-26;
--XO-O----OOOO--OOXOXXO-OOOOXXOOOOOXXOX-OXOXXXXX--XXXX----X-O-X- X; B7:-2; E1:-4; B1:-6; H2:-10; H5:-10; B2:-12; A2:-14; H3:-28; G1:-28; G2:-28;
--O-------OOO--X-XOOOOXXXXXXOXOX-XXOXOOXXXOXOOXX-OOOOO-X---OOO-- X; F1:+0; B2:+0; E1:+0; B1:-4; F2:-6; G7:-6; D1:-12; C8:-20; G8:-22; B8:-28;
-OXXXX----OXXO--XXOOXOOOXXXOOXOOXXOOXOOOXXXXOO-XX-XXO----------- X; G2:+10; A1:+4; G6:-10; H2:-12; F8:-12; E8:-12; G7:-24; G1:-24; B2:-30; F7:-34;
-XXX----X-XOO---XXOXOO--XOXOXO--XOOXOXXXXOOXXOX---OOOOO--XXXXX-- X; G3:+0; G2:-12; E1:-16; F2:-18; F1:-22; G4:-22; H6:-24; B7:-24; G8:-28;
-OOOOO----OOOO--OXXOOO---XXXOO--XXXXXXO-XXXOOO-OX-OOOO---OOOOO-- X; G6:-2; G3:-4; G4:-8; G7:-14; H5:-14; G2:-16; G1:-30; G8:-32;
--XX----O-XXOX--OOXOO---OXOXOOO-OOXXOOOXOOXXXOOX--XXXXOX--X--X-X X; G3:-4; B7:-6; E1:-8; H4:-10; F3:-10; H3:-10; B2:-14; A7:-22;
-XXXXXXX--XOOO----OXOOXX-OOXXOXX-OOOOOXX-X-XOOXX---O-X-X--OOOO-- X; E7:-8; A3:-8; A6:-12; B2:-12; G7:-12; G2:-12; A4:-14; C6:-20; A5:-22; B3:-28;
-------------O-O-OOOOOOOOOOOOXOOOXXOOOXO-XXXOXOO--XXXOXO--OXXXXO X; C2:-2; D2:-6; E2:-6; A3:-10; A2:-10; F1:-12; G2:-14; G1:-16; B2:-20; B8:-26;
--XXX-----XXXX-OOOXXOOOOOOOOOOXO-OOXXXXO-OOOXXXO---XOXX---X----- O; C7:+0; D8:-8; H8:-8; B2:-12; G1:-14; E8:-20; B1:-20; F8:-24; F1:-32; H7:-32; G8:-38;
---X-O----XXXO-XXXXXXXXXXOOXXOOXXOXOOOXXXXOOOO-XX--OOOO--------- O; B7:+0; B1:-2; E1:-4; C1:-6; G6:-8; G2:-10; A2:-22; B2:-24;
--OOOO--O-OOOO--OXXXOOO-OXXOXO--OOXXOXX-OOXXXX--O-XXX-----XX-O-- X; G2:-20; G4:-22; B7:-22; H3:-22; G1:-30; H2:-42; B1:-48;
--OOOO----OOOO---XOXXOOXOOXOOOOX-OOOOOXXXOOXXXXX--X-X----------- X; B2:+4; A5:+0; H2:-4; A3:-10; A7:-18; G2:-20; B7:-22; G1:-24; B1:-26;
O-OOOO--XOXXOX--XOOOXXX-XOOOXX--XOOXOX--XOXXX---X-XX------------ O; A8:+64; B1:+64; G1:+64; G5:+64; G6:+64; C8:+64; H3:+64; E8:+64; H4:+64; F7:+62; D8:+62; E7:+62; H2:+62; B8:+62; G2:+60; G4:+60; F6:+32;
";
