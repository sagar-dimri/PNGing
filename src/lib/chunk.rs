use super::chunk_type::ChunkType;
extern crate crc;
use crc::{Crc, CRC_32_ISO_HDLC};

use std::fmt::Display;
use std::string::FromUtf8Error;

pub struct Chunk {
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    chunk_checksum: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut ip_for_checksum = Vec::new();
        for chunk_type_byte in chunk_type.bytes() {
            ip_for_checksum.push(chunk_type_byte);
        }
        for ele in data.iter() {
            ip_for_checksum.push(*ele);
        }
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let checksum = crc.checksum(&ip_for_checksum[..]);
        Self {
            chunk_type,
            chunk_data: data,
            chunk_checksum: checksum,
        }
    }

    pub fn length(&self) -> u32 {
        self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data[..]
    }

    pub fn crc(&self) -> u32 {
        self.chunk_checksum
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        let res = String::from_utf8(self.chunk_data.clone())?;
        Ok(res)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length().to_be_bytes().to_vec()
            .iter()
            .cloned()
            .chain(self.chunk_type.bytes().iter().cloned())
            .chain(self.chunk_data.iter().cloned())
            .chain(self.chunk_checksum.to_be_bytes().to_vec().iter().cloned())
            .collect()
    }

}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = ChunkError;
    
    fn try_from(byte_vec: &Vec<u8>) -> Result<Self, Self::Error> {        
        match byte_vec.len() < 12 {
            true => Err(ChunkError::InvalidInput),
            false => {
                let length_arr = &byte_vec[..4];
                let data_len = u32::from_be_bytes([
                    length_arr[0], length_arr[1], length_arr[2], length_arr[3]
                    ]);
                match byte_vec.len() < (data_len + 12) as usize {
                    true => Err(ChunkError::InvalidLength),
                    false => {
                        let ip_len = byte_vec.len();
                        let mut data_vec = Vec::new();
                        for i in 0..(ip_len-12) {
                            data_vec.push(byte_vec[8+i]);
                        }
                        let chunk_type_arr = [byte_vec[4], byte_vec[5], byte_vec[6], byte_vec[7]];
                        let chunk_type = ChunkType::try_from(chunk_type_arr).unwrap();
                        
                        let mut crc_vec = Vec::new();
                        for i in 0..4 {
                            crc_vec.push(byte_vec[ip_len-4+i]);
                        }
                        let crc = u32::from_be_bytes([
                            crc_vec[0], crc_vec[1], crc_vec[2], crc_vec[3]
                        ]);
                        
                        // calculate checksum to validate the data
                        let ip_expected_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
                        let ip_expected_checksum = ip_expected_crc.checksum(&byte_vec[4..(ip_len-4)]);

                        match crc == ip_expected_checksum {
                            true => Ok(Self {
                                chunk_type,
                                chunk_data: data_vec,
                                chunk_checksum: crc
                            }),
                            false => Err(ChunkError::InvalidCrcReceived),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidInput,
    InvalidLength,
    InvalidCrcReceived,
}

#[allow(unused_variables)]
// fn main() {
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
// }

