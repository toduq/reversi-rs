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

    #[allow(unused)]
    pub fn swap(&self) -> Board {
        Board {
            me: self.opp,
            opp: self.me,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut map = HashMap::new();
        map.insert("ｏ".to_string(), self.me);
        map.insert("ｘ".to_string(), self.opp);
        write!(f, "{}", to_str(&map))
    }
}

pub fn to_str(map: &HashMap<String, u64>) -> String {
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
    buf.push_str("ーーーーーーーーーー\n");
    buf
}
