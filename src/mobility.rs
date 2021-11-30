use super::board;
use super::board::Board;
use std::collections::HashMap;

// https://techblog.cccmk.co.jp/entry/2020/12/07/170000
pub fn get_mobility(b: &Board) -> u64 {
    let masks = [
        (1, 0x7e7e7e7e7e7e7e7e), // right & left
        (7, 0x007e7e7e7e7e7e00), // upper-right & lower-left
        (8, 0x00ffffffffffff00), // up & down
        (9, 0x007e7e7e7e7e7e00), // upper-left & lowe-right
    ];
    let shifts = [std::ops::Shl::shl, std::ops::Shr::shr];
    let mut candidate = 0;

    for (n_shifts, mask) in masks.iter() {
        let mask = mask & b.opp;

        for shift in shifts.iter() {
            let mut bits = mask & shift(b.me, n_shifts);
            for _ in 0..5 {
                bits |= mask & shift(bits, n_shifts);
            }
            candidate |= shift(bits, n_shifts);
        }
    }

    candidate & !(b.me | b.opp)
}

// https://techblog.cccmk.co.jp/entry/2020/12/07/170000
fn get_flip(b: &Board, position: u64) -> u64 {
    let masks: [(i32, u64); 4] = [
        (1, 0xfefefefefefefefe),
        (7, 0x7f7f7f7f7f7f7f00),
        (8, 0xffffffffffffff00),
        (9, 0xfefefefefefefe00),
    ];
    let shifts = [std::ops::Shl::shl, std::ops::Shr::shr];
    let mut flip = 0;

    for (n_shifts, mut mask) in masks.iter() {
        for shift in shifts.iter() {
            let mut r = 0;
            let mut pos = mask & shift(position, n_shifts);
            while pos & b.opp != 0 {
                r |= pos;
                pos = mask & shift(pos, n_shifts);
            }
            if pos & b.me != 0 {
                flip |= r;
            }
            mask >>= n_shifts;
        }
    }
    flip
}

pub fn put(b: &Board, me_idx: u8) -> Board {
    let position = 1u64 << me_idx;
    let flip = get_flip(b, position);
    Board {
        me: b.opp ^ flip,
        opp: b.me ^ (position | flip),
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
            (171933433856, 35253361049600, 21, 35253091565568, 172205015040),
            (68719476736, 35356577103872, 12, 35356307619840, 68988964864),
            (35356307619840, 68988964864, 29, 68720529408, 35357112926208),
            (68720529408, 35357112926208, 30, 35357110829056, 69796368384),
            (35357110829056, 69796368384, 22, 69793222656, 35357118169088),
            (1073745920, 35425837645824, 14, 35425833451520, 1077956608),
            (3361235072, 9055819091939328, 4, 9055819091935232, 3361239184),
            (9055750104023040, 72349151376, 26, 71946498192, 9055750573785088),
            (71946498192, 9055750573785088, 44, 9055613134831616, 17801571496080),
            (9055613134831616, 17801571496080, 6, 17801570439312, 9055613135888448),
            (4569828968426750, 9078395931725824, 9, 9078395931463680, 4569828968689406),
            (9078395931463680, 4569828968689406, 17, 4569828966854398, 9078395933429760),
            (4569828966854398, 9078395933429760, 16, 9078395929366528, 4569828970983166),
            (9078395929366528, 4569828970983166, 33, 4569571272945406, 9078662217338880),
            (4569571272945406, 9078662217338880, 25, 9078644970360832, 4569588553477886),
            (9078644970360832, 4569588553477886, 0, 4569588418997502, 9078645104841217),
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
            println!("Base\n{}", print_movility(&b));
            println!("Idx : {}\n", idx);
            println!("Expected\n{}", print_movility(&expected.swap()));
            println!("Actual\n{}", print_movility(&actual.swap()));
            assert_eq!(actual, expected);
        }
    }
}
