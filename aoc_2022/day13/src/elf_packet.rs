#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ElfPacket {
    Literal(usize),
    List(Vec<ElfPacket>),
}
use std::cmp::Ordering;

impl std::cmp::PartialOrd for ElfPacket {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare(self, other))
    }
}

impl std::cmp::Ord for ElfPacket {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

fn compare_literal(l: &usize, r: &usize) -> Ordering {
    match (l, r) {
        (l, r) if l == r => Ordering::Equal,
        (l, r) if l < r => Ordering::Less,
        _ => Ordering::Greater,
    }
}

pub fn compare(left: &ElfPacket, right: &ElfPacket) -> Ordering {
    match (left, right) {
        (ElfPacket::Literal(l), ElfPacket::Literal(r)) => compare_literal(l, r),
        (ElfPacket::Literal(l), r) => compare(&ElfPacket::List(vec![ElfPacket::Literal(*l)]), r),
        (l, ElfPacket::Literal(r)) => compare(l, &ElfPacket::List(vec![ElfPacket::Literal(*r)])),
        (ElfPacket::List(l), ElfPacket::List(r)) => {
            for i in 0..(l.len().max(r.len())) {
                if i >= l.len() {
                    return Ordering::Less;
                }
                if i >= r.len() {
                    return Ordering::Greater;
                }
                let this_cmp = compare(&l[i], &r[i]);
                if this_cmp != Ordering::Equal {
                    return this_cmp;
                }
            }
            Ordering::Equal
        }
    }
}

impl std::fmt::Display for ElfPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElfPacket::Literal(usize) => write!(f, "{}", usize),
            ElfPacket::List(values) => {
                write!(f, "[")?;
                for value in values.iter() {
                    write!(f, "{}", value)?;
                    write!(f, ",")?;
                }
                write!(f, "]")
            }
        }
    }
}
