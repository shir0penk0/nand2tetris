pub mod parser;
use crate::parser::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    println!("filepath:{}", filepath);

    let mut parser = Parser::new(filepath);
    println!("{:#?}", parser);
    println!("------------------");

    loop {
        if parser.has_more_commands() == false {
            break;
        }
        parser = parser.advance();
        println!("{:#?}", parser);
        println!("{:#?}", parser.command_type());
        println!("symbol:{}", parser.symbol());
        println!("dest:{}", parser.dest());
        println!("comp:{}", parser.comp());
        println!("jump:{}", parser.jump());
        println!("------------------");
    }

}
