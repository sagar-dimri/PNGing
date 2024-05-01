use std::convert::TryFrom;
use std::str::FromStr;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType{
    byte_vec: Vec<u8>,
    validity: bool,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4]{
        let mut bytes_: [u8; 4] = [0; 4];
        for i in 0..4 {
            bytes_[i] = self.byte_vec[i];
        }
        return bytes_;
    }

    pub fn is_valid(&self) -> bool{
        self.validity
    }

    pub fn is_critical(&self) -> bool{
        self.byte_vec[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool{
        self.byte_vec[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool{
        self.byte_vec[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool{
        self.byte_vec[3].is_ascii_lowercase()
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(byte_stream: [u8; 4]) -> Result<Self, Self::Error> {
        let byte_vec = byte_stream.to_vec();
        let validity = byte_vec[2].is_ascii_uppercase();
        Ok(Self { byte_vec , validity })
    }
}

fn is_within_desired_ascii_range(s: &str) -> bool {
    if !(s.is_ascii()) {
        false
    } else {
        for c in s.chars() {
            if c < 'A' || c > 'z' || (c > 'Z' && c < 'a') {
                return false;
            }
        }
        true
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err(ChunkTypeError::InvalidChunkTypeLength)
        }else if !is_within_desired_ascii_range(s) {
            Err(ChunkTypeError::InvalidChunkTypeAscii)
        } else {
            let byte_vec = s.as_bytes().to_vec();
            let validity = byte_vec[2].is_ascii_uppercase();
            Ok(Self { byte_vec, validity })
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ascii_string: String = self.byte_vec.iter().map(|&byte| byte as char).collect();
        write!(f, "{}", ascii_string)
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidChunkTypeLength,
    InvalidChunkTypeAscii,
}


#[allow(unused_variables)]
// fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
// }

