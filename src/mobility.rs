use super::board;
use super::board::Board;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn get_mobility(b: &Board) -> u64 {
    let mut m = 0;
    for line in LINES.iter() {
        let mut occ = -1i8;
        for (i, pos) in line.iter().enumerate() {
            let opp = b.opp >> pos & 1 == 1;
            let me = b.me >> pos & 1 == 1;
            if me && occ >= 1 {
                m |= 1 << line[i - occ as usize - 1];
                occ = -1;
            } else if opp && occ >= 0 {
                occ += 1;
            } else if !me && !opp {
                occ = 0;
            } else {
                occ = -1;
            }
        }
    }
    m
}

pub fn get_flip(b: &Board, me_idx: u8) -> u64 {
    let mut flip = 0u64;
    for line in LINES_OF_INDEX[me_idx as usize].iter() {
        let mut occ = -1i8;
        for (i, pos) in line.iter().enumerate() {
            let opp = b.opp >> pos & 1 == 1;
            let me = b.me >> pos & 1 == 1;
            if me && occ >= 1 {
                for diff in 1..=occ {
                    flip |= 1 << line[i - diff as usize];
                }
                occ = -1;
            } else if opp && occ >= 0 {
                occ += 1;
            } else if !me && !opp {
                occ = 0;
            } else {
                occ = -1;
            }
        }
    }
    flip
}

pub fn put(b: &Board, me_idx: u8) -> Board {
    let flip = get_flip(b, me_idx);
    Board {
        me: b.opp & !flip,
        opp: b.me | flip | (1 << me_idx),
    }
}

pub fn print_movility(b: &Board) -> String {
    let mobility = get_mobility(b);
    let mut map = HashMap::new();
    map.insert("ｏ".to_string(), b.me);
    map.insert("ｘ".to_string(), b.opp);
    map.insert("＿".to_string(), mobility);
    board::to_str(&map)
}

#[allow(unused)]
pub fn print_flip(b: &Board, me_idx: u8) -> String {
    let flip = get_flip(b, me_idx);
    let mut map = HashMap::new();
    map.insert("＿".to_string(), flip);
    board::to_str(&map)
}

static LINES: Lazy<Vec<Vec<u8>>> = Lazy::new(|| {
    let mut one_way: Vec<Vec<u8>> = Vec::new();
    for i in 0..8 {
        one_way.push((0..8).map(|j| i * 8 + j).collect());
        one_way.push((0..8).map(|j| j * 8 + i).collect());
    }
    #[rustfmt::skip]
    let v = vec![(2,3),(3,4),(4,5),(5,6),(6,7),(7,8),(15,7),(23,6),(31,5),(39,4),(47,3)];
    for (i, len) in v {
        one_way.push((0..len).map(|j| i + (j * 7)).collect());
        let i = (i / 8 * 8) + (7 - i % 8);
        one_way.push((0..len).map(|j| i + (j * 9)).collect());
    }
    let mut two_way = Vec::new();
    for line in one_way {
        two_way.push(line.clone());
        two_way.push(line.into_iter().rev().collect());
    }
    two_way
});

#[allow(unused)]
static LINES_OF_INDEX: Lazy<Vec<Vec<Vec<u8>>>> = Lazy::new(|| {
    let mut v: Vec<Vec<Vec<u8>>> = (0..64).map(|_| vec![]).collect();
    for line in LINES.iter() {
        for idx in line {
            v[*idx as usize].push(line.clone());
        }
    }
    v
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mobility() {
        let cases: Vec<(u64, u64, u64)> = vec![
            (2319419934292831486, 71312739668481, 54187231568462080),
            (71308411146753, 2319419934326385918, 6363025526856089856),
            (17853105328272, 9055578781130816, 6944060251842973216),
            (134217728, 240786604032, 43981001981952),
            (35253091565568, 172201869312, 292595957760),
        ];
        for (me, opp, m) in cases {
            let b = Board { me, opp };
            println!("{}", print_movility(&b));
            assert_eq!(get_mobility(&b), m);
        }
    }

    #[test]
    fn test_flip() {
        #[rustfmt::skip]
        let cases: Vec<(u64, u64, u8, u64, u64)> = vec![
            (134217728, 240786604032, 45, 172067127296, 35253225783296),
            (172067127296, 35253225783296, 19, 35253091565568, 172201869312),
            (35253091565568, 172201869312, 20, 171933433856, 35253361049600),
            (171933433856, 35253361049600, 21, 68719476736, 35356577103872),
            (68719476736, 35356577103872, 12, 35356307619840, 68988964864),
            (35356307619840, 68988964864, 29, 68720529408, 35357112926208),
            (68720529408, 35357112926208, 30, 35357110829056, 69796368384),
            (35357110829056, 69796368384, 22, 1073745920, 35425837645824),
            (1073745920, 35425837645824, 14, 35425833451520, 1077956608),
            (3361235072, 9055819091939328, 4, 9055750104023040, 72349151376),
            (9055750104023040, 72349151376, 26, 71946498192, 9055750573785088),
            (71946498192, 9055750573785088, 44, 9055613134831616, 17801571496080),
            (9055613134831616, 17801571496080, 6, 17801570439312, 9055613135888448),
            (4569828968426750, 9078395931725824, 9, 9078395931463680, 4569828968689406),
            (9078395931463680, 4569828968689406, 17, 4569828966854398, 9078395933429760),
            (4569828966854398, 9078395933429760, 16, 9078395929366528, 4569828970983166),
            (9078395929366528, 4569828970983166, 33, 4569571272945406, 9078662217338880),
            (4569571272945406, 9078662217338880, 25, 9078644970360832, 4569588553477886),
            (9078644970360832, 4569588553477886, 0, 4534404046908670, 9113829476930049),
            (4534404046908670, 9113829476930049, 61, 71308411146753, 2319419934326385918),
            (71308411146753, 2319419934326385918, 32, 2319419934292831486, 71312739668481),
        ];
        for (me, opp, idx, next_me, next_opp) in cases {
            let b = Board { me, opp };
            let actual = put(&b, idx);
            let expected = Board {
                me: next_me,
                opp: next_opp,
            };
            println!("Actual\n{}", print_movility(&actual));
            println!("Expected\n{}", print_movility(&expected));
            assert_eq!(actual, expected);
        }
    }
}
