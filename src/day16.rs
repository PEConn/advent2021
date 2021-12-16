use std::collections::VecDeque;
use crate::day16::OperatorLength::{Bits, SubPackets};

// TODO: Give this a go with a parser.

#[derive(Debug, PartialEq)]
struct Header {
    version: u8,
    id: u8,
}

impl Header {
    /// Reads two three bit values, a version and an id.
    fn read<I: Iterator<Item = char>>(stream: &mut BitIterator<I>) -> Header {
        Header {
            version: stream.read(3) as u8,
            id: stream.read(3) as u8,
        }
    }
}

#[derive(Debug, PartialEq)]
struct NumberBody {
    value: u64,
}

impl NumberBody {
    /// Reads the stream in 5 bit chunks. If a chunk starts with a 1, the next 4 bits are
    /// appended to the working value and we continue reading. If a chunk starts with a 0,
    /// the nest 4 bits are appended to the working value and we finish reading.
    fn read<I: Iterator<Item = char>>(stream: &mut BitIterator<I>) -> NumberBody {
        // TODO: Do I need to include <I: Iterator<Item = char>> on every of my read methods?
        let mut value = 0;
        // All but the last bit start with a 1.
        while stream.next().unwrap() {
            value = value << 4;
            value = value | stream.read(4) as u64;
        }

        // Process the last bit as well.
        value = value << 4;
        value = value | stream.read(4) as u64;

        NumberBody { value }
    }
}

#[derive(Debug, PartialEq)]
enum PacketBody {
    Number(NumberBody),
    Operator(OperatorBody),
}

#[derive(Debug, PartialEq)]
struct Packet {
    header: Header,
    body: PacketBody,
}

impl Packet {
    /// Reads a header from the input, determines the type of the packet and reads the body as well.
    fn read<I: Iterator<Item=char>>(stream: &mut BitIterator<I>) -> Packet {
        let header = Header::read(stream);
        let id = header.id;

        Packet {
            header,
            body: if id == 4 {
                PacketBody::Number(NumberBody::read(stream))
            } else {
                PacketBody::Operator(OperatorBody::read(stream))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum OperatorLength {
    Bits(u32),
    SubPackets(u32),
}

#[derive(Debug, PartialEq)]
struct OperatorBody {
    length: OperatorLength,
    operands: Vec<Packet>,
}

impl OperatorBody {
    /// Reads the next bit, if it contains 1, reads the next 11 bits as
    /// OperatorLength::SubPackets, if it contains 0, reads the next 15 bits as
    /// OperatorLength::Bits.
    fn read<I: Iterator<Item=char>>(stream: &mut BitIterator<I>) -> OperatorBody {
        let length: OperatorLength = if stream.next().unwrap() {
            SubPackets(stream.read(11))
        } else {
            Bits(stream.read(15))
        };

        let mut operands : Vec<Packet> = Vec::new();

        match length {
            SubPackets(num_packets) => {
                for _ in 0..num_packets {
                    operands.push(Packet::read(stream));
                }
            },
            Bits(num_bits) => {
                let current_position = stream.bits_read;

                while stream.bits_read < current_position + num_bits {
                    operands.push(Packet::read(stream));
                }
            }
        }

        OperatorBody { length, operands }
    }
}

// TODO: Figure out generics a bit better to see if I can make this simpler.
struct BitIterator<I: Iterator<Item = char>> {
    inner: I,
    next_bits: VecDeque<bool>,
    bits_read: u32,
}

impl<I: Iterator<Item = char>> BitIterator<I> {
    fn new(inner: I) -> BitIterator<I> {
        BitIterator { inner, next_bits: VecDeque::new(), bits_read: 0 }
    }

    fn read(&mut self, no_bits: usize) -> u32 {
        let mut total = 0;
        for _ in 0..no_bits {
            total = total << 1;
            total = total | self.next().unwrap() as u32;
        }
        total
    }
}

impl<I: Iterator<Item = char>> Iterator for BitIterator<I> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_bits.is_empty() {
            if let Some(c) = self.inner.next() {
                let value = c.to_digit(16).unwrap();
                let binary_string = format!("{:04b}", value);

                self.next_bits = binary_string.chars().map(|c| c == '1').collect();
            } else {
                return None
            }
        }

        self.bits_read += 1;
        self.next_bits.pop_front()
    }
}

fn sum_versions(packet: &Packet) -> u32 {
    match &packet.body {
        PacketBody::Number(_) => packet.header.version as u32,
        PacketBody::Operator(operator_body) => {
            operator_body.operands.iter().map(sum_versions).sum::<u32>()
                + (packet.header.version as u32)
        }
    }
}

fn calculate(packet: &Packet) -> u64 {
    match &packet.body {
        PacketBody::Number(number) => number.value,
        PacketBody::Operator(operator) => {
            let mut operands = operator.operands.iter().map(calculate);
            match packet.header.id {
                0 => operands.sum(),
                1 => operands.product(),
                2 => operands.min().unwrap(),
                3 => operands.max().unwrap(),
                5 | 6 | 7 => {
                    let a = operands.next().unwrap();
                    let b = operands.next().unwrap();
                    assert_eq!(None, operands.next());

                    match packet.header.id {
                        5 => (a > b) as u64,
                        6 => (a < b) as u64,
                        7 => (a == b) as u64,
                        _ => panic!("Shouldn't happen"),
                    }
                },
                x => panic!("Unknown id: {}", x),
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut stream = BitIterator::new(input.chars());
    let root = Packet::read(&mut stream);
    sum_versions(&root)
}

pub fn part2(input: &str) -> u64 {
    let mut stream = BitIterator::new(input.chars());
    let root = Packet::read(&mut stream);
    calculate(&root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_iterator() {
        // let input = "D2FE28";
        let mut iter = BitIterator::new("1B".chars());

        // 0001
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());

        // 1011
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(false), iter.next());
        assert_eq!(Some(true), iter.next());
        assert_eq!(Some(true), iter.next());

        // Done
        assert_eq!(None, iter.next());
    }

    #[test]
    fn bit_iterator_read() {
        let mut iter = BitIterator::new("0157CF".chars());
        assert_eq!(0, iter.read(4));
        assert_eq!(1, iter.read(4));
        assert_eq!(5, iter.read(4));
        assert_eq!(7, iter.read(4));
        assert_eq!(12, iter.read(4));
        assert_eq!(15, iter.read(4));
    }

    #[test]
    fn read_number() {
        let input = "D2FE28";
        let mut stream = BitIterator::new(input.chars());

        let header = Header::read(&mut stream);
        assert_eq!(6, header.version);
        assert_eq!(4, header.id);

        let number = NumberBody::read(&mut stream);
        assert_eq!(2021, number.value);
    }

    #[test]
    fn read_operator1() {
        let input = "38006F45291200";
        let mut stream = BitIterator::new(input.chars());

        let header= Header::read(&mut stream);
        assert_eq!(1, header.version);
        assert_eq!(6, header.id);

        let operator = OperatorBody::read(&mut stream);
        assert_eq!(OperatorLength::Bits(27), operator.length);

        let expected = OperatorBody {
            length: OperatorLength::Bits(27),
            operands: vec![
                Packet {
                    header: Header { version: 6, id: 4 },
                    body: PacketBody::Number(NumberBody { value: 10 }),
                },
                Packet {
                    header: Header { version: 2, id: 4 },
                    body: PacketBody::Number(NumberBody { value: 20 }),
                }
            ]
        };
        assert_eq!(expected, operator);
    }

    #[test]
    fn read_operator2() {
        let input = "EE00D40C823060";
        let mut stream = BitIterator::new(input.chars());

        let header= Header::read(&mut stream);
        assert_eq!(7, header.version);
        assert_eq!(3, header.id);

        let operator = OperatorBody::read(&mut stream);

        let expected = OperatorBody {
            length: OperatorLength::SubPackets(3),
            operands: vec![
                Packet {
                    header: Header { version: 2, id: 4 },
                    body: PacketBody::Number(NumberBody { value: 1 }),
                },
                Packet {
                    header: Header { version: 4, id: 4 },
                    body: PacketBody::Number(NumberBody { value: 2 }),
                },
                Packet {
                    header: Header { version: 1, id: 4 },
                    body: PacketBody::Number(NumberBody { value: 3 }),
                }
            ]
        };
        assert_eq!(expected, operator);
    }

    #[test]
    fn given_examples_part1() {
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn given_example_part2() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));

        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(0, part2("9C005AC2F8F0"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}