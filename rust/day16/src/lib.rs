extern crate peg;

#[derive(Debug, PartialEq)]
struct Packet {
    packet_version: usize,
    packet_type: usize,
    length_type: LengthType,
    sub_packets: Option<Vec<Packet>>,
    literal_value: Option<usize>,
    bit_len: usize,
}

#[derive(Debug, PartialEq)]
enum Bit {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
enum LengthType {
    Bits,
    Packets,
}

pub struct Day16Setup {
    packets: Vec<Packet>,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        if let Some(sub_packets) = &self.sub_packets {
            return sub_packets
                .iter()
                .map(|packet| packet.sum_versions())
                .sum::<usize>()
                + self.packet_version;
        } else {
            self.packet_version
        }
    }

    fn evaluate(&self) -> usize {
        match self.packet_type {
            0 => self.sum(),
            1 => self.prod(),
            2 => self.min(),
            3 => self.max(),
            4 => self
                .literal_value
                .expect("Packet type of 4 must have a literal value defined"),
            5 => self.greater_than(),
            6 => self.less_than(),
            7 => self.equal(),
            _ => panic!("Invalid packet_type found"),
        }
    }

    fn get_evaluated_sub_packets(&self) -> Vec<usize> {
        if let Some(sub_packets) = &self.sub_packets {
            sub_packets
                .iter()
                .map(|packet| packet.evaluate())
                .collect::<Vec<usize>>()
        } else {
            vec![]
        }
    }

    fn get_evaluated_for_cmp(&self) -> (usize, usize) {
        let evaluated = self.get_evaluated_sub_packets();
        if evaluated.len() != 2 {
            panic!(
                "Invalid number of sub_packets for comparison. Expected 2 but got {}",
                evaluated.len()
            );
        } else {
            (evaluated[0], evaluated[1])
        }
    }

    fn sum(&self) -> usize {
        self.get_evaluated_sub_packets().into_iter().sum()
    }

    fn prod(&self) -> usize {
        self.get_evaluated_sub_packets().into_iter().product()
    }

    fn min(&self) -> usize {
        if let Some(ret) = self
            .get_evaluated_sub_packets()
            .into_iter()
            .reduce(std::cmp::min)
        {
            ret
        } else {
            panic!("Tried to take min of no sub_packets");
        }
    }

    fn max(&self) -> usize {
        if let Some(ret) = self
            .get_evaluated_sub_packets()
            .into_iter()
            .reduce(std::cmp::max)
        {
            ret
        } else {
            panic!("Tried to take max of no sub_packets");
        }
    }

    fn greater_than(&self) -> usize {
        let (left, right) = self.get_evaluated_for_cmp();
        match left > right {
            true => 1,
            false => 0,
        }
    }

    fn less_than(&self) -> usize {
        let (left, right) = self.get_evaluated_for_cmp();
        match left < right {
            true => 1,
            false => 0,
        }
    }

    fn equal(&self) -> usize {
        let (left, right) = self.get_evaluated_for_cmp();
        match left == right {
            true => 1,
            false => 0,
        }
    }
}

impl Day16Setup {
    fn get_packet_from_str_at_value(input_str: &str, packet_start: usize) -> Packet {
        if packet_start < input_str.len() * 4 {
            let packet_version =
                Day16Setup::get_bit_range_from_hex(input_str, packet_start, packet_start + 3);
            let packet_type =
                Day16Setup::get_bit_range_from_hex(input_str, packet_start + 3, packet_start + 6);
            if packet_type == 4 {
                // Literal type
                let mut literal_value = 0;
                let mut literal_start = packet_start + 6;
                loop {
                    literal_value <<= 4;
                    literal_value |= Day16Setup::get_bit_range_from_hex(
                        input_str,
                        literal_start + 1,
                        literal_start + 5,
                    );
                    if Day16Setup::get_bit_from_hex(input_str, literal_start) == Bit::Zero {
                        literal_start += 5;
                        break;
                    }
                    literal_start += 5;
                }
                Packet {
                    packet_version,
                    packet_type,
                    length_type: LengthType::Bits,
                    literal_value: Some(literal_value),
                    sub_packets: None,
                    bit_len: literal_start - packet_start,
                }
            } else {
                let length_type = Day16Setup::get_length_type(Day16Setup::get_bit_from_hex(
                    input_str,
                    packet_start + 6,
                ));
                let subpacket_start = match length_type {
                    LengthType::Bits => 22,
                    LengthType::Packets => 18,
                };
                let target_count = Day16Setup::get_bit_range_from_hex(
                    input_str,
                    packet_start + 7,
                    packet_start + subpacket_start,
                );
                let mut sub_packet_bits_consumed = 0;
                let mut sub_packets = vec![];
                while Day16Setup::sub_packets_to_read(
                    &length_type,
                    &sub_packets,
                    sub_packet_bits_consumed,
                    target_count,
                ) {
                    let sub_packet = Day16Setup::get_packet_from_str_at_value(
                        input_str,
                        packet_start + subpacket_start + sub_packet_bits_consumed,
                    );
                    sub_packet_bits_consumed += sub_packet.bit_len;
                    sub_packets.push(sub_packet);
                }
                Packet {
                    packet_version,
                    packet_type,
                    length_type,
                    literal_value: None,
                    sub_packets: Some(sub_packets),
                    bit_len: subpacket_start + sub_packet_bits_consumed,
                }
            }
        } else {
            panic!("Trying to address input_str longer than its starting point");
        }
    }

    fn sub_packets_to_read(
        length_type: &LengthType,
        sub_packets: &[Packet],
        sub_packet_bits_consumed: usize,
        target_count: usize,
    ) -> bool {
        match *length_type {
            LengthType::Bits => sub_packet_bits_consumed < target_count,
            LengthType::Packets => sub_packets.len() < target_count,
        }
    }

    fn get_length_type(bit: Bit) -> LengthType {
        match bit {
            Bit::One => LengthType::Packets,
            Bit::Zero => LengthType::Bits,
        }
    }

    fn hex_to_binary(c: &str) -> usize {
        if let Ok(val) = usize::from_str_radix(c, 16) {
            val
        } else {
            panic!("Invalid char for binary conversion");
        }
    }

    fn get_bit_from_hex(hex_array: &str, bit_location: usize) -> Bit {
        let char_to_select = bit_location / 4;
        let bit_within_char = 3 - (bit_location % 4);
        let char_as_int = Day16Setup::hex_to_binary(&hex_array[char_to_select..char_to_select + 1]);
        if char_as_int & (1 << bit_within_char) > 0 {
            Bit::One
        } else {
            Bit::Zero
        }
    }

    fn get_bit_range_from_hex(hex_array: &str, bit_start: usize, bit_end: usize) -> usize {
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
        Day16Setup {
            packets: vec![Day16Setup::get_packet_from_str_at_value(input_str, 0)],
        }
    }

    /// Calculate the part a response
    pub fn calculate_day_a(self: &Day16Setup) -> usize {
        self.packets
            .iter()
            .map(|packet| packet.sum_versions())
            .sum()
    }

    /// Calculate the part b response
    pub fn calculate_day_b(self: &Day16Setup) -> usize {
        self.packets.iter().map(|packet| packet.evaluate()).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::Bit;
    use crate::Day16Setup;

    #[test]
    fn test_parse_literal() {
        let day16_setup = Day16Setup::new("D2FE28");
        assert_eq!(day16_setup.packets[0].packet_version, 6);
        assert_eq!(day16_setup.packets[0].packet_type, 4);
        assert_eq!(day16_setup.packets[0].literal_value, Some(2021));
        assert_eq!(day16_setup.packets[0].sub_packets, None);
    }
    #[test]
    fn test_parse() {
        let day16_setup = Day16Setup::new("8A004A801A8002F478");
        assert_eq!(day16_setup.packets[0].packet_version, 4);
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap().len(),
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap()[0].packet_version,
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()[0]
                .packet_version,
            5
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            day16_setup.packets[0].sub_packets.as_ref().unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()[0]
                .sub_packets
                .as_ref()
                .unwrap()[0]
                .packet_version,
            6
        );
        /* let day16_setup = Day16Setup::new("620080001611562C8802118E34");
        assert_eq!(day16_setup.calculate_day_a(), 12);
        let day16_setup = Day16Setup::new("C0015000016115A2E0802F182340");
        assert_eq!(day16_setup.calculate_day_a(), 34);
        let day16_setup = Day16Setup::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(day16_setup.calculate_day_a(), 31); */
    }

    #[test]
    fn test_bit_from_hex() {
        let hex = String::from("1234");
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
        assert_eq!(day16_setup.calculate_day_a(), 23);
        let day16_setup = Day16Setup::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(day16_setup.calculate_day_a(), 31);
    }

    #[test]
    fn test_day_b() {
        let day16_setup = Day16Setup::new("C200B40A82");
        assert_eq!(day16_setup.calculate_day_b(), 3);
        let day16_setup = Day16Setup::new("04005AC33890");
        assert_eq!(day16_setup.calculate_day_b(), 54);
        let day16_setup = Day16Setup::new("880086C3E88112");
        assert_eq!(day16_setup.calculate_day_b(), 7);
        let day16_setup = Day16Setup::new("CE00C43D881120");
        assert_eq!(day16_setup.calculate_day_b(), 9);
        let day16_setup = Day16Setup::new("D8005AC2A8F0");
        assert_eq!(day16_setup.calculate_day_b(), 1);
        let day16_setup = Day16Setup::new("F600BC2D8F");
        assert_eq!(day16_setup.calculate_day_b(), 0);
        let day16_setup = Day16Setup::new("9C005AC2F8F0");
        assert_eq!(day16_setup.calculate_day_b(), 0);
        let day16_setup = Day16Setup::new("9C0141080250320F1802104A08");
        assert_eq!(day16_setup.calculate_day_b(), 1);
    }
}
