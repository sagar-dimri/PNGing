extern crate clap;

use clap::{App, AppSettings, SubCommand, Arg};
use std::ffi::OsString;
use std::str::FromStr;

use super::chunk_type::ChunkType;
use super::cli::{Encode, Decode, Remove, Print};

#[derive(Debug)]
pub enum Command {
    Encd(Encode),
    Decd(Decode),
    Remv(Remove),
    Prnt(Print),
}

pub fn parse_command() -> Command {
    parse_command_from(&mut std::env::args_os()).unwrap_or_else(|e| e.exit())
}

fn parse_command_from<I, T>(args: I) -> Result<Command, clap::Error>
where
    I: Iterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let arg_matches = App::new("PNGing")
                    .version("0.1")
                    .author("Sagar Dimri")
                    .about("This app lets you encode-decode message into/from a PNG file")
                    .setting(AppSettings::SubcommandRequired)
                    .subcommand(SubCommand::with_name("encode")
                        .about("Encodes a message into a PNG file")
                        .arg(
                            Arg::with_name("input_file")
                            .help("Input file name")
                            .long("input_file")
                            .short("ip")
                            .value_name("FILE")
                            .required(true)   
                        )
                        .arg(
                            Arg::with_name("chunk_type")
                            .help("Chunk type of the incoming message")
                            .long("chunk_type")
                            .short("c")
                            .value_name("CHUNK-TYPE")
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("message")
                            .help("Message that is to be encoded")
                            .long("msg")
                            .short("m")
                            .value_name("MESSAGE")
                            .required(true)
                        )
                        .arg(
                            Arg::with_name("output_file")
                            .help("Output file name, if ommited nothing is dumped to stdout/file ")
                            .long("output_file")
                            .short("op")
                            .value_name("FILE")
                            .required(false)
                        )
                    )
                    .subcommand(SubCommand::with_name("decode")
                        .about("Decodes a message from a PNG file")
                        .arg(
                            Arg::with_name("input_file")
                            .help("Input file name")
                            .long("input_file")
                            .short("ip")
                            .value_name("FILE")
                            .required(true)   
                        )
                        .arg(
                            Arg::with_name("chunk_type")
                            .help("Chunk type of the incoming message")
                            .long("chunk_type")
                            .short("c")
                            .value_name("CHUNK-TYPE")
                            .required(true)
                        )
                    )
                    .subcommand(SubCommand::with_name("remove")
                        .about("Removes message from a PNG file, if provided with a valid chunk-type")
                        .arg(
                            Arg::with_name("input_file")
                            .help("Input file name")
                            .long("input_file")
                            .short("ip")
                            .value_name("FILE")
                            .required(true)   
                        )
                        .arg(
                            Arg::with_name("chunk_type")
                            .help("Chunk type of the incoming message")
                            .long("chunk_type")
                            .short("c")
                            .value_name("CHUNK-TYPE")
                            .required(true)
                        )
                    )
                    .subcommand(SubCommand::with_name("print")
                        .about("Prints the PNG in a readable format")
                        .arg(
                            Arg::with_name("input_file")
                            .help("Input file name")
                            .long("input_file")
                            .short("ip")
                            .value_name("FILE")
                            .required(true)   
                        )
                    )
                    .get_matches_from_safe(args)?;
    
    if let Some(encode) = arg_matches.subcommand_matches("encode") {
        let ip_fp = OsString::from(encode.value_of("input_file").unwrap());
        let ct = ChunkType::from_str(encode.value_of("chunk_type").unwrap()).unwrap();
        let msg = encode.value_of("message").unwrap().to_string();
        if let Some(op_fp) = encode.value_of("output_file"){
            Ok(Command::Encd(
                Encode { 
                    input_file_path: ip_fp, 
                    chunk_type: ct,
                    message: msg, 
                    output_file_path: Some(OsString::from(op_fp)),
                }
            ))
        }else {
            Ok(Command::Encd(
                Encode { 
                    input_file_path: ip_fp, 
                    chunk_type: ct,
                    message: msg, 
                    output_file_path: None,
                }
            ))
        }
    } else if let Some(decode) = arg_matches.subcommand_matches("decode") {
        Ok(Command::Decd(
            Decode { 
                input_file_path: OsString::from(decode.value_of("input_file").unwrap()), 
                chunk_type: ChunkType::from_str(decode.value_of("chunk_type").unwrap()).unwrap()
            }
        ))
    } else if let Some(remove) = arg_matches.subcommand_matches("remove") {
        Ok(Command::Remv(
            Remove { 
                input_file_path: OsString::from(remove.value_of("input_file").unwrap()), 
                chunk_type: ChunkType::from_str(remove.value_of("chunk_type").unwrap()).unwrap()
            }
        ))
    } else if let Some(print_) = arg_matches.subcommand_matches("print") {
        Ok(Command::Prnt(
            Print { 
                input_file_path: OsString::from(print_.value_of("input_file").unwrap())
            }
        ))
    } else {
        panic!("This shouldn't happen {:?}", arg_matches);
    }
}