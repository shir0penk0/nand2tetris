use std::fs::File;
use std::io::{Write, BufWriter};
use regex::Regex; 
mod code;
mod parser;
use crate::code::*;
use crate::parser::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    // println!("filepath:{}", filepath);

    let mut parser = Parser::new(filepath);
    // println!("{:#?}", parser);
    // println!("------------------");


    let re = Regex::new(r"^(?:.*/)*(.+)\.asm$").unwrap();
    let cap = re.captures(filepath).unwrap();
    let output_name= format!("{}.hack",cap.get(1).unwrap().as_str());

    let mut writer = BufWriter::new(File::create(output_name).unwrap());

    loop {
        if parser.has_more_commands() == false {
            break;
        }
        parser = parser.advance();
        let cmd_type = parser.command_type().unwrap();
        let output_bin;

        if cmd_type == CommandType::ACommand {
            let symbol: i32 = parser.symbol().unwrap().parse().unwrap();
            output_bin = format!("0{:>015b}\n", symbol);
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
        } else if cmd_type == CommandType::LCommand {
            // TODO
            output_bin = "LCommand\n".to_string();
            // let symbol = parser.symbol().unwrap();
        } else {
            output_bin = "\n".to_string();
        }
        writer.write_all(output_bin.as_bytes()).unwrap();
        // println!("{:#?}", parser);
        // println!("{}", output_bin);
        // println!("------------------");
    }
}

