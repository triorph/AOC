extern crate peg;
use crate::elf_packet::ElfPacket;
use aoc_helpers::AOCFileOrParseError;

peg::parser! { pub grammar day13_parser() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().expect(&format!("Was expecting a number string {}", n)[..])}
    rule packet_number() -> ElfPacket
        = n:number() { ElfPacket::Literal(n) }
    rule packet() -> ElfPacket
        = "[" packets:packet_entry()** "," "]" { ElfPacket::List(packets) }
    rule packet_entry() -> ElfPacket
    = entry:(packet_number() / packet()) { entry }
    rule packet_pair() -> (ElfPacket, ElfPacket)
        = ep1:packet_entry() "\n" ep2:packet_entry() { (ep1, ep2) }
    rule line_of_packet_pairs() -> Vec<(ElfPacket,ElfPacket)>
        = packet_pairs:packet_pair() ++ ( "\n"* ) { packet_pairs }
    pub rule parse() -> Vec<(ElfPacket,ElfPacket)>
        = line_of_packet_pairs:line_of_packet_pairs() "\n" * {
             { line_of_packet_pairs }
        }
}}

pub fn parse_data(input: &str) -> Result<Vec<(ElfPacket, ElfPacket)>, AOCFileOrParseError> {
    if let Ok(ret) = day13_parser::parse(input) {
        Ok(ret)
    } else {
        Err(AOCFileOrParseError)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    use aoc_helpers::read_input_file;
    #[test]
    fn test_parse() {
        let input_str = read_input_file("data/test_data.txt").unwrap();
        let actual = day13_parser::parse(&input_str).expect("Should parse successfully");
        assert_eq!(actual.len(), 8);
        for line in actual.iter() {
            println!("actual: {}", line.0);
            println!("actual: {}", line.1);
        }
    }
}
