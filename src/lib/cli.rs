// This file has all the cli commands and underlying sub-commands
use std::ffi::OsString;

use super::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Encode {
    pub input_file_path: OsString,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output_file_path: Option<OsString>,
}

#[derive(Debug)]
pub struct Decode {
    pub input_file_path: OsString,
    pub chunk_type: ChunkType,
}

#[derive(Debug)]
pub struct Remove {
    pub input_file_path: OsString,
    pub chunk_type: ChunkType,
}

#[derive(Debug)]
pub struct Print {
    pub input_file_path: OsString,
}

