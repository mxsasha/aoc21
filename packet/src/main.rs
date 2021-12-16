use std::io::{self, Read};

#[derive(Debug)]
enum LengthType {
    BitLength(usize),
    NumberLength(usize),
}
#[derive(Debug)]
struct OperatorPacket {
    version: usize,
    length: LengthType,
    length_bits: usize,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
struct LiteralPacket {
    version: usize,
    value: usize,
    length_bits: usize,
}

#[derive(Debug)]
enum Packet {
    Operator(OperatorPacket),
    Literal(LiteralPacket),
}

impl OperatorPacket {
    fn new(binstr: &mut String, version: usize, packet_type: usize) -> Self {
        let length_type_id = binstr_to_usize(binstr.drain(..1).collect());
        let (length_type, bits_for_length): (fn(usize) -> LengthType, usize) = match length_type_id
        {
            0 => (LengthType::BitLength, 15),
            1 => (LengthType::NumberLength, 11),
            _ => panic!("Unknown length type: {}", length_type_id),
        };
        let length_value = binstr_to_usize(binstr.drain(..bits_for_length).collect());
        let length = length_type(length_value);
        let mut my_length = 6 + 1 + bits_for_length;

        let mut subpackets = vec![];
        if let LengthType::NumberLength(subpacket_count) = length {
            for _ in 0..subpacket_count {
                let subpacket = Packet::new(binstr);
                my_length += match &subpacket {
                    Packet::Operator(operator_subpacket) => operator_subpacket.length_bits,
                    Packet::Literal(literal_subpacket) => literal_subpacket.length_bits,
                };
                subpackets.push(subpacket);
            }
        }
        if let LengthType::BitLength(mut subpacket_remaining_bits) = length {
            while subpacket_remaining_bits > 0 {
                let subpacket = Packet::new(binstr);
                let subpacket_length = match &subpacket {
                    Packet::Operator(operator_subpacket) => operator_subpacket.length_bits,
                    Packet::Literal(literal_subpacket) => literal_subpacket.length_bits,
                };
                my_length += subpacket_length;
                subpacket_remaining_bits -= subpacket_length;

                subpackets.push(subpacket);
            }
        }
        OperatorPacket {
            version,
            length,
            subpackets,
            length_bits: my_length,
        }
    }
}

impl LiteralPacket {
    fn new(binstr: &mut String, version: usize) -> Self {
        let mut value_binstr = String::new();
        let mut nibble: String = binstr.drain(..5).collect();
        let mut length_bits = 5 + 6;
        value_binstr += &nibble[1..];
        while nibble.starts_with('1') {
            nibble = binstr.drain(..5).collect();
            value_binstr += &nibble[1..];
            length_bits += 5;
        }
        LiteralPacket {
            version,
            length_bits,
            value: binstr_to_usize(value_binstr),
        }
    }
}

impl Packet {
    fn new(binstr: &mut String) -> Self {
        let version = binstr_to_usize(binstr.drain(..3).collect());
        let packet_type = binstr_to_usize(binstr.drain(..3).collect());
        if packet_type == 4 {
            Packet::Literal(LiteralPacket::new(binstr, version))
        } else {
            Packet::Operator(OperatorPacket::new(binstr, version, packet_type))
        }
    }
    fn version_sum(&self) -> usize {
        let mut start = 0;
        if let Packet::Operator(operator_packet) = self {
            start += operator_packet.version;
            start += operator_packet
                .subpackets
                .iter()
                .map(|sp| sp.version_sum())
                .sum::<usize>();
        } else if let Packet::Literal(literal_packet) = self {
            start += literal_packet.version;
        }
        start
    }
}

fn binstr_to_usize(binstr: String) -> usize {
    usize::from_str_radix(&binstr, 2).unwrap()
}

fn hex_to_binstr(hex: &str) -> String {
    hex.chars().map(char_to_binstr).collect()
}

fn char_to_binstr(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn calculate(input: &str) -> usize {
    let packet = Packet::new(&mut hex_to_binstr(input.trim()));
    println!("{:?}", packet);
    packet.version_sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let score = calculate(&input);
    println!("result: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("38006F45291200"), 9);
        assert_eq!(calculate("EE00D40C823060"), 14);
        assert_eq!(calculate("8A004A801A8002F478"), 16);
        assert_eq!(calculate("620080001611562C8802118E34"), 12);
        assert_eq!(calculate("C0015000016115A2E0802F182340"), 23);
        assert_eq!(calculate("A0016C880162017C3686B18A3D4780"), 31);
    }
}
