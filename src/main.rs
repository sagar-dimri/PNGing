mod cli;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use std::str;

use std::{ffi::OsString, fs::File, io::{Read, Write}, path::PathBuf};

use commands::Command::{Encd, Decd, Remv, Prnt};
use cli::{Encode, Decode, Remove, Print};

use crate::{chunk::Chunk, png::Png};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn get_png_from_file(os_string: OsString) -> Png {
    let path = PathBuf::from(os_string);
    println!("{}", path.to_str().unwrap());
    let mut file = File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let contents_slice: &[u8] = &contents;
    let mut png = Png::try_from(contents_slice).unwrap();
    png
}

pub fn execute_encoding(encode_args: Encode){
    let mut png = get_png_from_file(encode_args.input_file_path);
    let chunk_containg_msg = Chunk::new(encode_args.chunk_type, encode_args.message.into_bytes());
    png.append_chunk(chunk_containg_msg);
    if let Some(op_path) = encode_args.output_file_path {
        let mut file2 = File::create(PathBuf::from(op_path)).unwrap();
        file2.write_all(&png.as_bytes()).unwrap();
    }
}

pub fn execute_decoding(decode_args: Decode){
    let png = get_png_from_file(decode_args.input_file_path);
    if let Some(chunk) =  png.chunk_by_type(str::from_utf8(&decode_args.chunk_type.bytes()).unwrap()) {
        println!("hidden message is: \n {}", chunk);
    }else {
        println!("no message found associated with this chunk type");
    }

}
pub fn execute_removing(remove_args: Remove){
    let mut png = get_png_from_file(remove_args.input_file_path);
    let removed_chunk = png.remove_chunk(str::from_utf8(&remove_args.chunk_type.bytes()).unwrap()).unwrap();
    println!("removed chunk: {}", removed_chunk);
}
pub fn execute_printing(print_args: Print){
    let png = get_png_from_file(print_args.input_file_path);
    println!("[PNG]: \n {}", png);
}

fn main() -> Result<()> {
    match commands::parse_command()
    {
        Encd(encode) => execute_encoding(encode),
        Decd(decode) => execute_decoding(decode),
        Remv(remove) => execute_removing(remove),
        Prnt(print_) => execute_printing(print_),
    }
    Ok(())
}
