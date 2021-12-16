fn parse_input(input: &str) -> impl Iterator<Item = bool> + '_ {
    input.chars().flat_map(|c| {
        let bits = match c as u8 {
            b'0'..=b'9' => c as u8 - b'0',
            b'A'..=b'F' => c as u8 - b'A' + 10,
            _ => panic!("Non Hex Digit!"),
        };
        [bits & 8 != 0, bits & 4 != 0, bits & 2 != 0, bits & 1 != 0]
    })
}

pub fn parse_number(bit_stream: &mut impl Iterator<Item = bool>, bits: usize) -> usize {
    bit_stream
        .take(bits)
        .fold(0, |number, bit| (number << 1) | bit as usize)
}

pub fn parse_literal(bit_stream: &mut impl Iterator<Item = bool>) -> usize {
    let mut bit_list = Vec::new();
    loop {
        let bits = bit_stream.take(5).collect::<Vec<_>>();
        #[allow(clippy::match_ref_pats)]
        if !match bits.as_slice() {
            &[c, b1, b2, b3, b4] => {
                bit_list.push([b1, b2, b3, b4]);
                c
            }
            _ => {
                panic!("Out of bits while reading literal block!")
            }
        } {
            break;
        }
    }
    let bits = bit_list.len() * 4;
    parse_number(&mut bit_list.into_iter().flatten(), bits)
}

pub fn parse_packet_kind(bit_stream: &mut impl Iterator<Item = bool>) -> PacketKind {
    let id = parse_number(bit_stream, 3) as u8;

    if id == 4 {
        PacketKind::Literal(parse_literal(bit_stream))
    } else {
        let op = match id {
            0 => Operator::Sum,
            1 => Operator::Prod,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => panic!("Unknown Operator"),
        };
        let mut arguments = vec![];
        let op_data = match bit_stream.next() {
            None => panic!("Out of bits while reading length type ID"),
            Some(false) => {
                let length = parse_number(bit_stream, 15);
                let mut data = bit_stream.take(length as usize).peekable();
                while data.peek().is_some() {
                    let mut box_iter: Box<dyn Iterator<Item = bool>> = Box::new(&mut data);
                    arguments.push(parse_packet(&mut box_iter))
                }
                OperatorData::TotalLength
            }
            Some(true) => {
                let count = parse_number(bit_stream, 11);
                arguments.reserve(count);
                for _ in 0..count {
                    arguments.push(parse_packet(bit_stream))
                }
                OperatorData::PacketCount
            }
        };
        PacketKind::Operator {
            op,
            op_data,
            arguments,
        }
    }
}

pub fn parse_packet(bit_stream: &mut impl Iterator<Item = bool>) -> Packet {
    let version = parse_number(bit_stream, 3) as u8;
    let kind = parse_packet_kind(bit_stream);
    Packet { version, kind }
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    kind: PacketKind,
}

#[derive(Debug)]
pub enum PacketKind {
    Literal(usize),
    Operator {
        op: Operator,
        op_data: OperatorData,
        arguments: Vec<Packet>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Sum,
    Prod,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
pub enum OperatorData {
    TotalLength,
    PacketCount,
}

pub fn part1(input: &str) -> u32 {
    let mut iter = parse_input(input);

    let packet = parse_packet(&mut iter);

    let mut to_process = vec![packet];

    let mut sum = 0;
    while let Some(Packet { version, kind }) = to_process.pop() {
        sum += version as u32;
        match kind {
            PacketKind::Literal(_) => {}
            PacketKind::Operator {
                op: _,
                op_data: _,
                mut arguments,
            } => to_process.append(&mut arguments),
        }
    }
    sum
}

pub fn eval(packet: &Packet) -> usize {
    match &packet.kind {
        PacketKind::Literal(val) => *val,
        PacketKind::Operator {
            op: Operator::Sum,
            arguments,
            ..
        } => arguments.iter().map(eval).sum(),
        PacketKind::Operator {
            op: Operator::Prod,
            op_data: _,
            arguments,
        } => arguments.iter().map(eval).product(),
        PacketKind::Operator {
            op: Operator::Min,
            op_data: _,
            arguments,
        } => arguments.iter().map(eval).min().unwrap(),
        PacketKind::Operator {
            op: Operator::Max,
            op_data: _,
            arguments,
        } => arguments.iter().map(eval).max().unwrap(),
        PacketKind::Operator {
            op: Operator::GreaterThan,
            op_data: _,
            arguments,
        } => {
            let first = eval(&arguments[0]);
            let second = eval(&arguments[1]);
            if first > second {
                1
            } else {
                0
            }
        }
        PacketKind::Operator {
            op: Operator::LessThan,
            op_data: _,
            arguments,
        } => {
            let first = eval(&arguments[0]);
            let second = eval(&arguments[1]);
            if first < second {
                1
            } else {
                0
            }
        }
        PacketKind::Operator {
            op: Operator::EqualTo,
            op_data: _,
            arguments,
        } => {
            let first = eval(&arguments[0]);
            let second = eval(&arguments[1]);
            if first == second {
                1
            } else {
                0
            }
        }
    }
}

pub fn part2(input: &str) -> usize {
    let mut iter = parse_input(input);

    let packet = parse_packet(&mut iter);

    eval(&packet)
}

#[test]
fn part1_example_2() {
    let input = "D2FE28";
    assert_eq!(part1(input), 6);
}

#[test]
fn part1_example_1() {
    let input = "38006F45291200";
    assert_eq!(part1(input), 1 + 6 + 2);
}

#[test]
fn part1_example0() {
    let input = "EE00D40C823060";
    assert_eq!(part1(input), 7 + 2 + 4 + 1);
}

#[test]
fn part1_example1() {
    let input = "8A004A801A8002F478";
    assert_eq!(part1(input), 4 + 1 + 5 + 6);
}

#[test]
fn part1_example2() {
    let input = "620080001611562C8802118E34";
    assert_eq!(part1(input), 3 + 9);
}

#[test]
fn part1_example3() {
    let input = "C0015000016115A2E0802F182340";
    assert_eq!(part1(input), 23);
}

#[test]
fn part1_example4() {
    let input = "A0016C880162017C3686B18A3D4780";
    assert_eq!(part1(input), 31);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day16.txt"));
    assert_eq!(part1(input), 986);
}

#[test]
fn part2_example_2() {
    let input = "C200B40A82";
    assert_eq!(part2(input), 1 + 2);
}

#[test]
fn part2_example_1() {
    let input = "04005AC33890";
    assert_eq!(part2(input), 6 * 9);
}

#[test]
fn part2_example0() {
    let input = "880086C3E88112";
    assert_eq!(part2(input), [7, 8, 9].into_iter().min().unwrap());
}

#[test]
fn part2_example1() {
    let input = "CE00C43D881120";
    assert_eq!(part2(input), [7, 8, 9].into_iter().max().unwrap());
}

#[test]
fn part2_example2() {
    let input = "D8005AC2A8F0";
    assert_eq!(part2(input), (5 < 15) as usize);
}

#[test]
fn part2_example3() {
    let input = "F600BC2D8F";
    assert_eq!(part2(input), (5 > 15) as usize);
}

#[test]
fn part2_example4() {
    let input = "9C005AC2F8F0";
    assert_eq!(part2(input), (5 == 15) as usize);
}

#[test]
fn part2_example5() {
    let input = "9C0141080250320F1802104A08";
    assert_eq!(part2(input), (1 + 3 == 2 * 2) as usize);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day16.txt"));
    assert_eq!(part2(input), 18234816469452);
}
