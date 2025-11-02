use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BitReader {
    data: Rc<[u64]>,
    word_index: usize,
    bit_index: u8,
    len_bits: usize,
}

impl BitReader {
    fn new(data: Vec<u64>, len_bits: usize) -> Self {
        Self {
            data: data.into(),
            word_index: 0,
            bit_index: 0,
            len_bits,
        }
    }

    fn peek_field(&self, bit_size: u8) -> Option<u64> {
        let data_index = self.word_index;
        let bit_index = self.bit_index;

        if self.word_index * 64 + usize::from(self.bit_index) + usize::from(bit_size)
            > self.len_bits
        {
            return None;
        }

        Some(if bit_index + bit_size > 64 {
            let word1 = self.data[data_index];
            let word2 = self.data[data_index + 1];
            (word1 & !(!0 << (64 - bit_index))) << (bit_index + bit_size - 64)
                | word2 >> (128 - bit_index - bit_size)
        } else if bit_size == 64 {
            self.data[data_index] // No need to shift or mask
        } else {
            let word = self.data[data_index];
            (word >> (64 - bit_index - bit_size)) & !(!0 << bit_size)
        })
    }

    fn advance_field(&mut self, bit_size: usize) {
        self.word_index += (usize::from(self.bit_index) + bit_size) / 64;
        self.bit_index = (self.bit_index + u8::try_from(bit_size % 64).unwrap()) % 64;
    }

    fn read_field(&mut self, bit_size: u8) -> Option<u64> {
        let value = self.peek_field(bit_size)?;
        self.advance_field(usize::from(bit_size));
        Some(value)
    }

    fn sub_reader(&mut self, len_bits: usize) -> Option<Self> {
        if self.word_index * 64 + usize::from(self.bit_index) + len_bits > self.len_bits {
            return None;
        }
        let mut sub_reader = self.clone(); // Clones the Rc, without duplicating the data
        sub_reader.len_bits = self.word_index * 64 + usize::from(self.bit_index) + len_bits;
        self.advance_field(len_bits);
        Some(sub_reader)
    }

    fn read<T>(&mut self) -> Result<T, T::Error>
    where
        T: ReadBits,
    {
        T::read_bits(self)
    }
}

impl FromStr for BitReader {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let mut size_bits = 0;
        for start in (0..s.len()).step_by(16) {
            let mut word = 0_u64;
            for ch in s[start..].bytes().chain(std::iter::repeat(b'0')).take(16) {
                word = (word << 4)
                    | u64::from(match ch {
                        b'0'..=b'9' => ch - b'0',
                        b'A'..=b'F' => ch - b'A' + 0xA,
                        _ => return Err(ParseError::SyntaxError),
                    });
                size_bits += 4;
            }
            data.push(word);
        }
        Ok(Self::new(data, size_bits))
    }
}

trait ReadBits: Sized {
    type Error;

    fn read_bits(r: &mut BitReader) -> Result<Self, Self::Error>;
}

#[derive(Debug, Error)]
enum PacketError {
    #[error("Invalid packet type")]
    InvalidPacketType,
    #[error("Reached end of input")]
    EndOfInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeID {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

impl Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ReadBits for TypeID {
    type Error = PacketError;

    fn read_bits(r: &mut BitReader) -> Result<Self, Self::Error> {
        Ok(match r.read_field(3).ok_or(PacketError::EndOfInput)? {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::Equal,
            _ => return Err(PacketError::InvalidPacketType),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Sum(u8, Children),
    Product(u8, Children),
    Minimum(u8, Children),
    Maximum(u8, Children),
    Literal(u8, Literal),
    GreaterThan(u8, Children),
    LessThan(u8, Children),
    Equal(u8, Children),
}

impl Packet {
    fn version(&self) -> u64 {
        match self {
            Self::Sum(v, _)
            | Self::Product(v, _)
            | Self::Minimum(v, _)
            | Self::Maximum(v, _)
            | Self::Literal(v, _)
            | Self::GreaterThan(v, _)
            | Self::LessThan(v, _)
            | Self::Equal(v, _) => u64::from(*v),
        }
    }

    fn version_sum(&self) -> u64 {
        (match self {
            Self::Literal(..) => 0,
            Self::Sum(_, children)
            | Self::Product(_, children)
            | Self::Minimum(_, children)
            | Self::Maximum(_, children)
            | Self::GreaterThan(_, children)
            | Self::LessThan(_, children)
            | Self::Equal(_, children) => children.version_sum(),
        }) + self.version()
    }

    fn evaluate(&self) -> u64 {
        match self {
            Self::Sum(_, children) => children.0.iter().map(Self::evaluate).sum(),
            Self::Product(_, children) => children.0.iter().map(Self::evaluate).product(),
            Self::Minimum(_, children) => children.0.iter().map(Self::evaluate).min().unwrap_or(0),
            Self::Maximum(_, children) => children.0.iter().map(Self::evaluate).max().unwrap_or(0),
            Self::Literal(_, literal) => literal.0,
            Self::GreaterThan(_, children)
            | Self::LessThan(_, children)
            | Self::Equal(_, children)
                if children.0.len() != 2 =>
            {
                panic!("Non-binary comparison!")
            }
            Self::GreaterThan(_, children) => {
                u64::from(children.0[0].evaluate() > children.0[1].evaluate())
            }
            Self::LessThan(_, children) => {
                u64::from(children.0[0].evaluate() < children.0[1].evaluate())
            }
            Self::Equal(_, children) => {
                u64::from(children.0[0].evaluate() == children.0[1].evaluate())
            }
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Sum(_, children) => write!(f, "Sum({children})"),
            Self::Product(_, children) => write!(f, "Product({children})"),
            Self::Minimum(_, children) => write!(f, "Minimum({children})"),
            Self::Maximum(_, children) => write!(f, "Maximum({children})"),
            Self::Literal(_, literal) => write!(f, "{literal}"),
            Self::GreaterThan(_, children) => write!(f, "GreaterThan({children})"),
            Self::LessThan(_, children) => write!(f, "LessThan({children})"),
            Self::Equal(_, children) => write!(f, "Equal({children})"),
        }
    }
}

impl ReadBits for Packet {
    type Error = PacketError;

    fn read_bits(r: &mut BitReader) -> Result<Self, Self::Error> {
        let version_value = r.read_field(3).ok_or(PacketError::EndOfInput)?;
        let version = u8::try_from(version_value).unwrap();
        let packet_type: TypeID = r.read()?;
        Ok(match packet_type {
            TypeID::Sum => Self::Sum(version, r.read()?),
            TypeID::Product => Self::Product(version, r.read()?),
            TypeID::Minimum => Self::Minimum(version, r.read()?),
            TypeID::Maximum => Self::Maximum(version, r.read()?),
            TypeID::Literal => Self::Literal(version, r.read()?),
            TypeID::GreaterThan => Self::GreaterThan(version, r.read()?),
            TypeID::LessThan => Self::LessThan(version, r.read()?),
            TypeID::Equal => Self::Equal(version, r.read()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Literal(u64);

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ReadBits for Literal {
    type Error = PacketError;

    fn read_bits(r: &mut BitReader) -> Result<Self, Self::Error> {
        let mut val = 0;
        loop {
            let chunk = r.read_field(5).ok_or(PacketError::EndOfInput)?;
            val = (val << 4) | (chunk & 0xF);
            if chunk & 0x10 == 0 {
                break;
            }
        }
        Ok(Self(val))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Children(Vec<Packet>);

impl Children {
    fn version_sum(&self) -> u64 {
        self.0.iter().map(Packet::version_sum).sum()
    }
}

impl Display for Children {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for child in &self.0 {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

impl ReadBits for Children {
    type Error = PacketError;

    fn read_bits(r: &mut BitReader) -> Result<Self, Self::Error> {
        let mut children: Vec<Packet> = Vec::new();
        if r.read_field(1).ok_or(PacketError::EndOfInput)? == 0 {
            let len_bits = r.read_field(15).ok_or(PacketError::EndOfInput)?;
            let len_bits_usize = usize::try_from(len_bits).unwrap();
            let mut inner_reader = r
                .sub_reader(len_bits_usize)
                .ok_or(PacketError::EndOfInput)?;
            loop {
                match inner_reader.read() {
                    Err(PacketError::EndOfInput) => break,
                    Ok(child) => children.push(child),
                    Err(err) => return Err(err),
                }
            }
        } else {
            let child_count = r.read_field(11).ok_or(PacketError::EndOfInput)?;
            for _ in 0..child_count {
                children.push(r.read()?);
            }
        }
        Ok(Self(children))
    }
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Result<BitReader, ParseError> {
    input.parse()
}

#[aoc(day16, part1)]
fn part_1(reader: &BitReader) -> u64 {
    let mut reader = reader.clone();
    let packet: Packet = reader.read().unwrap();

    packet.version_sum()
}

#[aoc(day16, part2)]
fn part_2(reader: &BitReader) -> u64 {
    let mut reader = reader.clone();
    let packet: Packet = reader.read().unwrap();

    packet.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_bitreader() {
        let mut reader = BitReader::from_str("3372F9333015AA0C91E03F4809C38B8F").unwrap();
        assert_eq!(reader.read_field(8).unwrap(), 0x33);
        assert_eq!(reader.read_field(12).unwrap(), 0x72F);
        assert_eq!(reader.read_field(2).unwrap(), 2); // 9 = 0b1001 -> 0b10 (2), 0b01 (1)
        assert_eq!(reader.read_field(2).unwrap(), 1);
        assert_eq!(reader.read::<TypeID>().unwrap(), TypeID::Product);
        assert_eq!(reader.read_field(64).unwrap(), 0x9980_AD50_648F_01FA);
    }

    #[test_case("D2FE28" => 6)]
    #[test_case("38006F45291200" => 9)]
    #[test_case("EE00D40C823060" => 14)]
    #[test_case("8A004A801A8002F478" => 16)]
    #[test_case("620080001611562C8802118E34" => 12)]
    #[test_case("C0015000016115A2E0802F182340" => 23)]
    #[test_case("A0016C880162017C3686B18A3D4780" => 31)]
    fn test_part_1(input: &str) -> u64 {
        let reader = input.parse().unwrap();
        part_1(&reader)
    }

    #[test_case("C200B40A82" => 3)]
    #[test_case("04005AC33890" => 54)]
    #[test_case("880086C3E88112" => 7)]
    #[test_case("CE00C43D881120" => 9)]
    #[test_case("D8005AC2A8F0" => 1)]
    #[test_case("F600BC2D8F" => 0)]
    #[test_case("9C005AC2F8F0" => 0)]
    #[test_case("9C0141080250320F1802104A08" => 1)]
    fn test_part_2(input: &str) -> u64 {
        let reader = input.parse().unwrap();
        part_2(&reader)
    }
}
