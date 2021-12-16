extern crate peg;
use itertools::Itertools;

struct Packet {
    packet_version: usize,
    packet_type: usize,
    length_type: LengthType,
    sub_packets: Option<Vec<Packet>>,
    literal_value: Option<usize>,
}

#[derive(Debug, PartialEq)]
enum Bit {
    One,
    Zero,
}

enum LengthType {
    Bits,
    Packets,
}

pub struct Day16Setup {
    packets: Vec<Packet>,
}

impl Day16Setup {
    fn hex_to_binary(c: char) -> usize {
        let c_as_str: String = [c].iter().collect();
        if let Ok(val) = usize::from_str_radix(&c_as_str[..], 16) {
            val
        } else {
            panic!("Invalid char for binary conversion");
        }
    }

    fn get_bit_from_hex(hex_array: &[char], bit_location: usize) -> Bit {
        let char_to_select = bit_location / 4;
        let bit_within_char = 3 - (bit_location % 4);
        let char_as_int = Day16Setup::hex_to_binary(hex_array[char_to_select]);
        println!(
            "Got char_as_int {} for bit_location {} {} {}",
            char_as_int, bit_location, bit_within_char, char_to_select
        );
        if char_as_int & (1 << bit_within_char) > 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    }

    fn get_bit_range_from_hex(hex_array: &[char], bit_start: usize, bit_end: usize) -> usize {
        let mut ret = 0;
        for bit_location in bit_start..bit_end {
            ret <<= 1;
            if Day16Setup::get_bit_from_hex(hex_array, bit_location) == Bit::One {
                ret |= 1;
            }
        }
        ret
    }

    /// Generates a new Day16Setup object to be calculated upon
    ///
    /// Inputs: the full string contents of the input data set.
    ///
    /// Returns: A new Day16Setup object, with methods `calculate_day_a` and `calculate_day_b`
    /// available
    pub fn new(input_str: &str) -> Day16Setup {

        Day16Setup { packets: vec![] }
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day16Setup) -> usize {
        0
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day16Setup) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::Bit;
    use crate::Day16Setup;

    /* #[test]
    fn test_parse() {
        let day16_setup = Day16Setup::new("8A004A801A8002F478");
        assert_eq!(day16_setup.packets[0].packet_type, 4);
        assert_eq!(day16_setup.packets[0].sub_packets.unwrap().len(), 1);
        assert_eq!(
            day16_setup.packets[0].sub_packets.unwrap()[0].packet_type,
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.unwrap()[0]
                .sub_packets
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.unwrap()[0]
                .sub_packets
                .unwrap()[0]
                .packet_type,
            5
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.unwrap()[0]
                .sub_packets
                .unwrap()[0]
                .sub_packets
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.unwrap()[0]
                .sub_packets
                .unwrap()[0]
                .sub_packets
                .unwrap()[0]
                .packet_type,
            6
        );
        /* let day16_setup = Day16Setup::new("620080001611562C8802118E34");
        assert_eq!(day16_setup.calculate_day_a(), 12);
        let day16_setup = Day16Setup::new("C0015000016115A2E0802F182340");
        assert_eq!(day16_setup.calculate_day_a(), 34);
        let day16_setup = Day16Setup::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(day16_setup.calculate_day_a(), 31); */
    } */

    #[test]
    fn test_bit_from_hex() {
        let hex = vec!['1', '2', '3', '4'];
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 0), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 1), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 2), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 3), Bit::One);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 4), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 5), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 6), Bit::One);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 7), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 8), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 9), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 10), Bit::One);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 11), Bit::One);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 12), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 13), Bit::One);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 14), Bit::Zero);
        assert_eq!(Day16Setup::get_bit_from_hex(&hex, 15), Bit::Zero);
    }

    #[test]
    fn test_day_a() {
        let day16_setup = Day16Setup::new("8A004A801A8002F478");
        assert_eq!(day16_setup.calculate_day_a(), 16);
        let day16_setup = Day16Setup::new("620080001611562C8802118E34");
        assert_eq!(day16_setup.calculate_day_a(), 12);
        let day16_setup = Day16Setup::new("C0015000016115A2E0802F182340");
        assert_eq!(day16_setup.calculate_day_a(), 34);
        let day16_setup = Day16Setup::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(day16_setup.calculate_day_a(), 31);
    }

    #[test]
    fn test_day_b() {
        let day16_setup = Day16Setup::new(include_str!("../test_data.txt"));
        assert_eq!(day16_setup.calculate_day_b(), 0);
    }
}
