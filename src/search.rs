use super::board::Board;
use super::mobility;
use std::cmp::min;
use std::cmp::Ordering;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub idx: u8,
    pub score: i8,
    pub searched: u32,
    pub game_end: bool,
}

fn result_of(idx: u8, score: i8, searched: u32, game_end: bool) -> SearchResult {
    SearchResult {
        idx,
        score,
        searched,
        game_end,
    }
}

const MAX_DEPTH: u8 = 20;

pub fn complete_search(b: &Board) -> SearchResult {
    let depth = (b.me | b.opp).count_zeros() as u8 + 1;
    let result = rec_search(b, 0, depth, -i8::MAX, i8::MAX, &None).unwrap();
    println!("searched depth#{}, result = {:?}", depth, result);
    result
}

pub fn find_best_move(b: &Board, ms: u64) -> u8 {
    if let Some(r) = one_mobility_check(b) {
        println!("no choice but {}", r);
        return r;
    }
    let mut result = result_of(u8::MAX, 0, 0, false);
    let deadline = Instant::now().checked_add(Duration::from_millis(ms));
    for depth in 5..=MAX_DEPTH {
        if let Some(r) = rec_search(b, 0, depth, -i8::MAX, i8::MAX, &deadline) {
            result = r;
            println!("searched depth#{}, result = {:?}", depth, result);
            if result.game_end {
                println!("search completed depth#{}", depth);
                return result.idx;
            }
        } else {
            println!("aborted depth#{}", depth);
            break;
        }
    }
    result.idx
}

fn one_mobility_check(b: &Board) -> Option<u8> {
    let m = mobility::get_mobility(b);
    if m.count_ones() == 1 {
        Some(m.trailing_zeros() as u8)
    } else {
        None
    }
}

fn rec_search(
    b: &Board,
    depth: u8,
    max_depth: u8,
    alpha: i8,
    beta: i8,
    deadline: &Option<Instant>,
) -> Option<SearchResult> {
    let occupied = !(b.me | b.opp);
    if depth >= max_depth || occupied == 0 {
        return Some(result_of(u8::MAX, evaluate(b), 1, occupied == 0));
    }

    let mobility = mobility::get_mobility(b);
    if mobility == 0 {
        let passed = b.pass();
        if mobility::get_mobility(&passed) == 0 {
            // game end
            return Some(result_of(u8::MAX, evaluate(b), 1, true));
        } else {
            // pass
            return rec_search(&passed, depth, max_depth, -beta, -alpha, &None).map(|r| {
                SearchResult {
                    score: -r.score,
                    ..r
                }
            });
        }
    }

    let mut best: SearchResult = result_of(u8::MAX, -i8::MAX, 0, false);
    let mut moves: [u8; 30] = [0; 30];
    if max_depth - depth <= 6 {
        naive_ordering(mobility, &mut moves);
    } else {
        fastest_first_ordering(b, mobility, &mut moves);
    }

    for idx in moves {
        if idx == u8::MAX {
            break;
        }
        if search_for_idx(b, idx, depth, max_depth, alpha, beta, &mut best) {
            break;
        }
        if let Some(d) = deadline {
            if Instant::now().saturating_duration_since(*d).as_nanos() > 0 {
                return None;
            }
        }
    }
    Some(best)
}

#[inline]
fn search_for_idx(
    b: &Board,
    idx: u8,
    depth: u8,
    max_depth: u8,
    alpha: i8,
    beta: i8,
    best: &mut SearchResult,
) -> bool {
    let next_board = mobility::put(b, idx);
    let result = rec_search(
        &next_board,
        depth + 1,
        max_depth,
        -beta,
        min(-best.score, -alpha),
        &None,
    )
    .unwrap();
    let score = -result.score;
    best.searched += result.searched;
    if score > best.score {
        best.idx = idx;
        best.score = score;
        best.game_end = result.game_end;
    }
    if score >= beta {
        return true;
    }
    false
}

fn naive_ordering(mobility: u64, moves: &mut [u8; 30]) {
    let mut i = 0;
    let masks: [u64; 3] = [
        0x8100_0000_0000_0081, // corner
        0x3C3C_FFFF_FFFF_3C3C, // others
        0x42C3_0000_0000_C342, // c + x
    ];
    for mask in masks {
        let mut masked_mobility = mobility & mask;
        while masked_mobility != 0 {
            let idx = masked_mobility.trailing_zeros() as u8;
            masked_mobility ^= 1 << idx;
            moves[i] = idx;
            i += 1;
        }
    }
    moves[i] = u8::MAX;
}

fn fastest_first_ordering(b: &Board, mobility: u64, moves: &mut [u8; 30]) {
    let mut m: [(u8, u8); 30] = [(u8::MAX, u8::MAX); 30];
    let mut i = 0;
    for idx in 0..64 {
        if mobility >> idx & 1 == 0 {
            continue;
        }
        let next_board = mobility::put(b, idx);
        m[i] = (idx, mobility::get_mobility(&next_board).count_ones() as u8);
        i += 1;
    }
    m[0..i].sort_by(|a, b| a.1.cmp(&b.1));
    for (j, (idx, _)) in m.into_iter().enumerate() {
        moves[j] = idx;
    }
    moves[i] = u8::MAX;
}

fn evaluate(b: &Board) -> i8 {
    let m = b.me.count_ones() as i8;
    let o = b.opp.count_ones() as i8;
    if m + o < 50 {
        // evaluate by mobility
        mobility::get_mobility(b).count_ones() as i8
            - mobility::get_mobility(&b.pass()).count_ones() as i8
    } else {
        // evaluate by score
        // Using `m - o` as score is not appropriate in order to pass FFO.
        // https://github.com/primenumber/issen-rs/blob/master/src/board.rs#L231
        match m.cmp(&o) {
            Ordering::Greater => 64 - 2 * o,
            Ordering::Less => -64 + 2 * m,
            Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_move() {
        let b = Board::new();
        println!("{}", b);
        assert!(vec![29, 43, 45].contains(&find_best_move(&b, 100)));
    }

    #[test]
    fn test_one_mobility_check() {
        let b = Board {
            me: 114633790074399,
            opp: 18446066489965990112,
        };
        println!("{}", b);
        let m = mobility::get_mobility(&b);
        println!("mobility is {}, {:#064b}", m, m);
        assert_eq!(one_mobility_check(&b), Some(49));
    }
}
