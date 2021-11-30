use super::mobility;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub me: u64,
    pub opp: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            me: 1 << 27,
            opp: (1 << 28) | (7 << 35),
        }
    }

    pub fn pass(&self) -> Board {
        Board {
            me: self.opp,
            opp: self.me,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut map = HashMap::new();
        map.insert("ｘ".to_string(), self.me);
        map.insert("ｏ".to_string(), self.opp);
        map.insert("＿".to_string(), mobility::get_mobility(self));
        write!(
            f,
            "{}\nSer : {}\nDbg : {:?}\n",
            to_str(&map),
            serialize(self),
            self
        )
    }
}

fn to_str(map: &HashMap<String, u64>) -> String {
    let mut buf = "ーーーーーーーーーー\n".to_string();
    for y in 0..8 {
        buf.push_str("| ");
        for x in 0..8 {
            let idx = y * 8 + x;
            let mut found = false;
            for (s, b) in map {
                if b >> idx & 1 == 1 {
                    buf.push_str(s);
                    found = true;
                }
            }
            if !found {
                buf.push('　');
            }
        }
        buf.push_str(" |\n")
    }
    buf.push_str("ーーーーーーーーーー");
    buf
}

// --XXXXX--OOOXX-O-OOOXXOX-OXOXOXXOXXXOXXX--XOXOXX-XXXOOO--OOOOO--
pub fn parse(s: &str) -> Board {
    let mut me = 0u64;
    let mut opp = 0u64;
    for (idx, c) in s.as_bytes().iter().enumerate() {
        match c {
            b'X' => me |= 1 << idx,
            b'O' => opp |= 1 << idx,
            _ => {}
        }
    }
    Board { me, opp }
}

pub fn serialize(b: &Board) -> String {
    let mut s = "".to_owned();
    for idx in 0..64 {
        if b.me >> idx & 1 == 1 {
            s.push('X');
        } else if b.opp >> idx & 1 == 1 {
            s.push('O');
        } else {
            s.push('-');
        }
    }
    s
}
