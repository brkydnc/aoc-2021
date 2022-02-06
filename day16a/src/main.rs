use Packet::*;

enum Packet {
    Literal {
        version: u8,
        value: usize,
    },
    Operator {
        version: u8,
        subpackets: Vec<Packet>,
    }
}

struct Parser;

impl Parser {
    fn parse(msg: &str) -> Packet {
        Self::parse_packet(&Self::convert_to_binary(msg)).0
    }

    fn convert_to_binary(msg: &str) -> String {
        msg
            .chars()
            .fold(String::with_capacity(msg.len() * 4), |s, c| s + hex_to_binary(c))
    }

    fn parse_packet(msg: &str) -> (Packet, usize) {
        let version = u8::from_str_radix(&msg[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&msg[3..6], 2).unwrap();

        if type_id != 4 {
            let lti = if &msg[6..7] == "0" { 15 } else { 11 };
            let lti_value = usize::from_str_radix(&msg[7..7 + lti], 2).unwrap();

            let mut index = 7 + lti;
            let mut subpackets = vec![];

            if lti == 15 {
                let end = index + lti_value;

                loop {
                    let (packet, subend) = Self::parse_packet(&msg[index..end]);
                    subpackets.push(packet);
                    index += subend;
                    if end == index { break; }
                }

                (Operator { version, subpackets }, end)
            } else {
                for _ in 0..lti_value {
                    let (packet, subend) = Self::parse_packet(&msg[index..]);
                    subpackets.push(packet);
                    index += subend;
                }

                (Operator { version, subpackets }, index)
            }
        } else {
            let mut value = 0;
            let mut index = 6;

            loop {
                let (prefix, n) = (&msg[index..index + 5]).split_at(1);

                value *= 16;
                value += usize::from_str_radix(n, 2).unwrap();

                index += 5;
                if prefix == "0" { break; }
            }

            (Literal { version, value }, index)
        }
    }
}

const fn hex_to_binary(hex: char) -> &'static str {
    match hex {
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
        _  => unreachable!()
    }
}

fn add_versions(packet: &Packet) -> u32 {
    match packet {
        Literal { version, .. } => { *version as u32 },
        Operator { version, subpackets }=> {
            let mut sum = *version as u32;
            for subpacket in subpackets {
                sum += add_versions(subpacket)
            }
            sum
        },
    }
}

fn main() {
    let message = include_str!("../input.txt")
        .trim();

    let packet = Parser::parse(message);
    println!("{}", add_versions(&packet));
}
