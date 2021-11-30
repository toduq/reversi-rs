use super::board::Board;
use super::mobility;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
struct SearchResult {
    idx: u8,
    score: i8,
    searched: u32,
    game_end: bool,
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

pub fn find_best_move(b: &Board, ms: u64) -> u8 {
    if let Some(r) = one_mobility_check(b) {
        println!("no choice but {}", r);
        return r;
    }
    let mut result = result_of(u8::MAX, 0, 0, false);
    let deadline = Instant::now().checked_add(Duration::from_millis(ms));
    for depth in 2..=MAX_DEPTH {
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
    if depth >= max_depth {
        return Some(result_of(u8::MAX, evaluate(b), 1, false));
    }
    let mobility = mobility::get_mobility(b);
    if mobility == 0 {
        if mobility::get_mobility(&b.swap()) == 0 {
            // game end
            return Some(result_of(u8::MAX, evaluate(b), 1, true));
        } else {
            // pass
            let r = rec_search(&b.swap(), depth + 1, max_depth, -beta, -alpha, &None).unwrap();
            return Some(SearchResult {
                score: -r.score,
                ..r
            });
        }
    }

    let mut best: SearchResult = result_of(u8::MAX, alpha, 0, false);

    for idx in ordered_mobility(b, mobility) {
        let next_board = mobility::put(b, idx);
        let result =
            rec_search(&next_board, depth + 1, max_depth, -beta, -best.score, &None).unwrap();
        let score = -result.score;
        best.searched += result.searched;
        if score > best.score {
            best.idx = idx;
            best.score = score;
            best.game_end = result.game_end;
        }
        if score > beta {
            return Some(best);
        }
        if let Some(d) = deadline {
            if Instant::now().saturating_duration_since(*d).as_nanos() > 0 {
                return None;
            }
        }
    }
    Some(best)
}

fn ordered_mobility(b: &Board, mobility: u64) -> Vec<u8> {
    let mut mobilities: Vec<(u8, u32)> = (0..64)
        .filter(|idx| mobility >> idx & 1 == 1)
        .map(|idx| {
            let next_board = mobility::put(b, idx);
            (idx, mobility::get_mobility(&next_board).count_ones())
        })
        .collect();
    mobilities.sort_by(|a, b| b.1.cmp(&a.1));
    mobilities.iter().map(|m| m.0).collect()
}

fn evaluate(b: &Board) -> i8 {
    let m_stones = b.me.count_ones();
    let o_stones = b.opp.count_ones();
    if m_stones + o_stones < 50 {
        // evaluate by mobility
        mobility::get_mobility(b).count_ones() as i8
            - mobility::get_mobility(&b.swap()).count_ones() as i8
    } else {
        // evaluate by score
        (m_stones - o_stones) as i8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_mobility() {
        let b = Board::new();
        let actual = ordered_mobility(&b, mobility::get_mobility(&b));
        let expected = vec![29, 43, 45];
        assert_eq!(actual, expected);
    }

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
