use crate::board::Board;
use crate::mobility;
use crate::search;
use crate::search::SearchResult;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug)]
struct MTree {
    board: Board,
    played: u32,
    won: u32,
    children: HashMap<u8, MTree>,
}

impl MTree {
    fn new(b: Board) -> MTree {
        MTree {
            board: b,
            played: 0,
            won: 0,
            children: HashMap::new(),
        }
    }
}

impl std::fmt::Display for MTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "".to_owned();
        fmt_in(self, 0, 0, true, &mut s);
        write!(f, "{}", s)
    }
}

fn fmt_in(t: &MTree, idx: u8, depth: u8, best: bool, s: &mut String) {
    s.push_str(&format!(
        "{}{} played:{} won:{}{}\n",
        "  ".repeat(depth as usize),
        idx,
        t.played,
        t.won,
        if best { "  *" } else { "" },
    ));
    if t.children.is_empty() {
        return;
    }
    let best_idx = t
        .children
        .iter()
        .map(|(idx, t)| (idx, t.won))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    for (idx, child) in t.children.iter() {
        fmt_in(child, *idx, depth + 1, best && best_idx == idx, s);
    }
}

const SEARCH_TRIAL: u32 = 100000;
const EXPAND_LIMIT: u32 = 1000;

#[allow(unused)]
pub fn search(b: &Board) {
    let mut tree = MTree::new(b.clone());
    expand(&mut tree);
    for _ in 0..SEARCH_TRIAL {
        search_in(&mut tree);
    }
    println!("{}", tree);
}

fn search_in(t: &mut MTree) -> SearchResult {
    if t.children.is_empty() && t.played < EXPAND_LIMIT {
        // playout
        t.played += 1;
        let result = playout(&t.board);
        if result.score > 0 {
            t.won += 1;
        }
        return result;
    }
    if t.children.is_empty() {
        // expand
        expand(t);
    }

    // evaluate
    let all_play_sum: f32 = t.children.iter().map(|(_, t)| t.played as f32).sum();
    let mut arms: Vec<(u8, f32)> = t
        .children
        .iter()
        .map(|(idx, t)| {
            let eff = t.won as f32 / (t.played + 1) as f32;
            let search = (2.0 * (all_play_sum + 1.0).ln() / (t.played as f32 + 1.0)).sqrt();
            (*idx, eff + search)
        })
        .collect();
    arms.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let selected_idx = arms[0].0;
    let child = t.children.get_mut(&selected_idx).unwrap();
    t.played += 1;
    let result = search_in(child);
    if result.score > 0 {
        t.won += 1;
    }
    result
}

fn expand(t: &mut MTree) {
    if !t.children.is_empty() {
        panic!("monte_carlo::expand() called, but children is not empty");
    }
    let m = mobility::get_mobility(&t.board);
    for idx in mobility::iter(m) {
        let next = mobility::put(&t.board, idx);
        let child = MTree::new(next);
        t.children.insert(idx, child);
    }
}

const PLAYOUT_SEARCH_LIMIT: u32 = 8;

fn playout(board: &Board) -> SearchResult {
    let mut b = board.clone();
    let mut passes = false;
    loop {
        if (b.me | b.opp).count_zeros() <= PLAYOUT_SEARCH_LIMIT {
            break;
        }
        let m = mobility::get_mobility(&b);
        if m == 0 {
            if passes {
                break;
            }
            passes = true;
            b = b.pass();
            continue;
        }
        passes = false;

        let mut moves: Vec<u8> = mobility::iter(m).collect();
        moves.shuffle(&mut rand::thread_rng());
        b = mobility::put(&b, moves[0]);
    }

    search::complete_search(&b)
}
