use regex::Regex;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::num::ParseIntError;
mod code;
mod parser;
mod symbol_table;
use crate::code::Code;
use crate::parser::{CommandType, Parser};
use crate::symbol_table::SymbolTable;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let mut symbol_tb = SymbolTable::new();

    // First loop for set the symbol table up
    {
        let mut parser = Parser::new(filepath);
        let mut rom_address: u32 = 0;
        loop {
            if parser.has_more_commands() == false {
                break;
            }
            parser.advance();
            let cmd_type = parser.command_type().unwrap();
            match cmd_type {
                CommandType::ACommand => rom_address += 1,
                CommandType::CCommand => rom_address += 1,
                CommandType::LCommand => {
                    let s = parser.symbol().unwrap();
                    symbol_tb.add_entry(s.to_string(), rom_address);
                }
            }
        }
    }

    let mut parser = Parser::new(filepath);
    let mut ram_address: u32 = 16;

    let re = Regex::new(r"^(?:.*/)*(.+)\.asm$").unwrap();
    let cap = re.captures(filepath).unwrap();
    let output_name = format!("{}.hack", cap.get(1).unwrap().as_str());
    let mut writer = BufWriter::new(File::create(output_name).unwrap());

    // Second loop for compiling
    loop {
        if parser.has_more_commands() == false {
            break;
        }
        parser.advance();
        let cmd_type = parser.command_type().unwrap();
        let output_bin;

        if cmd_type == CommandType::ACommand {
            let symbol_result: Result<u32, ParseIntError> = parser.symbol().unwrap().parse();
            match symbol_result {
                Ok(v) => {
                    output_bin = format!("0{:>015b}\n", v);
                    writer.write_all(output_bin.as_bytes()).unwrap();
                }
                Err(_) => {
                    let key = parser.symbol().unwrap();
                    if symbol_tb.contains(&key.to_string()) {
                        let a = symbol_tb.get_address(&key.to_string()).unwrap();
                        output_bin = format!("0{:>015b}\n", a);
                        writer.write_all(output_bin.as_bytes()).unwrap();
                    } else {
                        symbol_tb.add_entry(key.to_string(), ram_address);
                        output_bin = format!("0{:>015b}\n", ram_address);
                        writer.write_all(output_bin.as_bytes()).unwrap();
                        ram_address += 1;
                    }
                }
            }
        } else if cmd_type == CommandType::CCommand {
            let dest = Code::dest(parser.dest().unwrap());
            let comp = Code::comp(parser.comp().unwrap());
            let jump = Code::jump(parser.jump().unwrap());
            output_bin = format!(
                "111{comp}{dest}{jump}\n",
                comp = comp,
                dest = dest,
                jump = jump
            );
            writer.write_all(output_bin.as_bytes()).unwrap();
        } else if cmd_type == CommandType::LCommand {
            // noop
        }
    }
}
