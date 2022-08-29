const LITERAL_TAG: usize = 4;

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        operands: Vec<Packet>,
    }
}

fn byte_to_hex(byte: &u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None
    }
}

fn byte_to_bits(byte: u8) -> [u8; 4] {
    [byte >> 3 & 1, byte >> 2 & 1, byte >> 1 & 1, byte & 1]
}

fn take(bits: &[u8], n: usize) -> Option<(usize, &[u8])> {
    let mut result = 0;
    for i in 0..n {
        result <<= 1;
        result |= *bits.get(i)? as usize;
    }
    Some((result, &bits[n..]))
}

fn parse_packets(bits: &[u8]) -> Option<(Packet, &[u8])> {
    let (version, bits) = take(bits, 3)?;
    let (type_id, mut bits) = take(bits, 3)?;
    if type_id == LITERAL_TAG {
        let mut value = 0;
        loop {
            let (mark, next) = take(bits, 1)?;
            let (part, next) = take(next, 4)?;
            bits = next;
            value = value << 4 | part;
            if mark == 0 {
                break;
            }
        }
        let packet = Packet::Literal {
            version,
            value
        };
        Some((packet, bits))
    } else {
        let (length_type_id, mut bits) = take(bits, 1)?;
        let operands = match length_type_id {
            0 => {
                let (subpacket_length, next) = take(bits, 15)?;
                let mut subpackets = vec![];
                bits = &next.get(subpacket_length..)?;
                let mut next = next.get(0..subpacket_length)?;
                while let Some((p, n)) = parse_packets(next) {
                    subpackets.push(p);
                    next = n;
                }
                subpackets
            }
            1 => {
                let (num_subpackets, next) = take(bits, 11)?;
                bits = next;
                let mut subpackets = Vec::with_capacity(num_subpackets);
                for _ in 0..num_subpackets {
                    let (packet, next) = parse_packets(bits)?;
                    subpackets.push(packet);
                    bits = next;
                }
                subpackets
            },
            _ => unreachable!(),
        };
        let packet = Packet::Operator {
            version,
            type_id,
            operands,
        };
        Some((packet, bits))
    }
}

fn part1(packet: &Packet) -> usize {
    match packet {
        Packet::Literal{ version, .. } => *version,
        Packet::Operator{ version, operands, .. } => *version as usize + operands.iter().map(part1).sum::<usize>()
    }
}

fn part2(packet: &Packet) -> usize {
    match packet {
        Packet::Literal{ value, .. } => *value,
        Packet::Operator{ type_id, operands, .. } => match type_id {
            0 => operands.iter().map(part2).sum(),
            1 => operands.iter().map(part2).product(),
            2 => operands.iter().map(part2).min().unwrap(),
            3 => operands.iter().map(part2).max().unwrap(),
            5 => if part2(&operands[0]) > part2(&operands[1]) {
                1
            } else {
                0
            },
            6 => if part2(&operands[0]) < part2(&operands[1]) {
                1
            } else {
                0
            },
            7 => if part2(&operands[0]) == part2(&operands[1]) {
                1
            } else {
                0
            },
            _ => unreachable!(),
        }
    }
}

fn to_bits(v: &[u8]) -> Vec<u8> {
    v.iter().filter_map(byte_to_hex).flat_map(byte_to_bits).collect()
}

fn main() {
    let bits: Vec<u8> = to_bits(include_bytes!("input.txt"));
    let (packets, _) = parse_packets(&bits).unwrap();
    println!("Part 1: {}", part1(&packets));
    println!("Part 2: {}", part2(&packets));
}
