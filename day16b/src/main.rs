use std::cmp::Ordering;

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal
}

type LiteralValue = usize;

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: LiteralValue,
    },
    Operator {
        version: u8,
        operation: Operation,
        subpackets: Vec<Packet>,
    }
}

impl Packet {
    fn evaluate(&self) -> LiteralValue {
        use Packet::*;
        use Operation::*;

        match self {
            Literal { value, .. } => *value,
            Operator { operation, subpackets, .. } => {
                let iter = subpackets.iter().map(|p| p.evaluate());
                let result = match operation {
                    Sum => iter.sum(),
                    Product => iter.product(),
                    Min => iter.min().unwrap(),
                    Max => iter.max().unwrap(),
                    Greater => (&subpackets[0]).cmp(&subpackets[1]),
                    Less => (&subpackets[1]).cmp(&subpackets[0]),
                    Equal => (&subpackets[0]).cmp(&subpackets[1]),
                };

                result
            }
        }
    }

    fn cmp(&self, other: &Self) -> LiteralValue {
        match self.evaluate().cmp(&other.evaluate()) {
            Ordering::Greater => 1,
            Ordering::Less => 0,
            Ordering::Equal => 1,
        }
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
        use Packet::*;
        use Operation::*;

        let version = u8::from_str_radix(&msg[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&msg[3..6], 2).unwrap();

        if type_id != 4 {
            let operation = match type_id {
                0 => Sum,
                1 => Product,
                2 => Min,
                3 => Max,
                5 => Greater,
                6 => Less,
                7 => Equal,
                _ => unreachable!()
            };

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

                (Operator { version, operation, subpackets }, end)
            } else {
                for _ in 0..lti_value {
                    let (packet, subend) = Self::parse_packet(&msg[index..]);
                    subpackets.push(packet);
                    index += subend;
                }

                (Operator { version, operation, subpackets }, index)
            }
        } else {
            let mut binary_integer = String::new();
            let mut index = 6;

            loop {
                let (prefix, n) = (&msg[index..index + 5]).split_at(1);
                binary_integer.push_str(n);
                index += 5;
                if prefix == "0" { break; }
            }

            let value = LiteralValue::from_str_radix(&binary_integer, 2).unwrap();

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

fn main() {
    let message = include_str!("../input.txt")
    // let message = ""
        .trim();

    let packet = Parser::parse(message);
    dbg!(&packet);
    println!("{}", packet.evaluate());
}
