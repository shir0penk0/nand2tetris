use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(Debug)]
pub struct Parser {
    line_vec: Vec<String>,
    line_cnt: usize,
    command: Option<Command>,
}

impl Parser {
    pub fn new(path: &str) -> Parser {
        let f = File::open(path).expect("file not found");
        let reader = BufReader::new(f);

        let mut p = Parser {
            line_vec: Vec::new(),
            line_cnt: 0,
            command: None,
        };
        for line in reader.lines() {
            let l = line.unwrap();

            // Skip an empty line
            if l.is_empty() {
                continue;
            }

            // Skip a comment line
            let re = Regex::new(r"^//.*$").unwrap();
            if re.is_match(&l) {
                continue;
            }

            p.line_vec.push(l);
        }
        p
    }

    pub fn has_more_commands(&self) -> bool {
        // self.line_vec.is_empty() == false
        self.line_cnt < self.line_vec.len()
    }

    pub fn advance(&mut self) {
        if self.has_more_commands() {
            self.line_cnt += 1;
            self.command = Some(Command::new(&self.line_vec[self.line_cnt-1][..]).unwrap());
        }
    }

    pub fn command_type(&self) -> Option<CommandType> {
        let c = self.command.as_ref();
        Some(c?.cmd_type)
    }

    pub fn symbol(&self) -> Option<&str> {
        let c = self.command.as_ref();
        match c?.cmd_type {
            CommandType::ACommand => Some(&c?.symbol),
            CommandType::LCommand => Some(&c?.symbol),
            CommandType::CCommand => None,
        }
    }

    pub fn dest(&self) -> Option<&str> {
        let c = self.command.as_ref();
        match c?.cmd_type {
            CommandType::CCommand => Some(&c?.dest),
            _ => None,
        }
    }

    pub fn comp(&self) -> Option<&str> {
        let c = self.command.as_ref();
        match c?.cmd_type {
            CommandType::CCommand => Some(&c?.comp),
            _ => None,
        }
    }

    pub fn jump(&self) -> Option<&str> {
        let c = self.command.as_ref();
        match c?.cmd_type {
            CommandType::CCommand => Some(&c?.jump),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Command {
    cmd_type: CommandType,
    symbol: String,
    dest: String,
    comp: String,
    jump: String,
}

impl Command {
    fn new(current_line: &str) -> Result<Command, io::Error> {
        // Return a type A command object if the current line matches type A command syntax
        let re_a_cmd = Regex::new(r"^\s*(?:@([\w_\.\$:]+))\s*(?://.*)*$").unwrap();
        if re_a_cmd.is_match(current_line) {
            let cap = re_a_cmd.captures(current_line).unwrap();
            let symbol = cap.get(1).unwrap().as_str();
            return Ok(Command {
                cmd_type: CommandType::ACommand,
                symbol: symbol.to_string(),
                dest: "".to_string(),
                comp: "".to_string(),
                jump: "".to_string(),
            });
        }

        // Return a type C command object if the current line matches type C command syntax
        let re_c_cmd = Regex::new(r"^\s*(?:(M|D|MD|A|AM|AD|AMD)=){0,1}((?:0|1|-1|D|A|!D|!A|-D|-A|D\+1|A\+1|D-1|A-1|D\+A|D-A|A-D|D\&A|D\|A|M|!M|-M|M\+1|M-1|D\+M|D-M|M-D|D\&M|D\|M){1})(?:;(JGT|JEQ|JGE|JLT|JNE|JLE|JMP)){0,1}\s*(?://.*)*$").unwrap();
        if re_c_cmd.is_match(current_line) {
            let cap = re_c_cmd.captures(current_line).unwrap();
            let dest = match cap.get(1) {
                Some(dest) => dest.as_str(),
                None => "null",
            };

            let comp = cap.get(2).unwrap().as_str();

            let jump = match cap.get(3) {
                Some(jump) => jump.as_str(),
                None => "null",
            };

            return Ok(Command {
                cmd_type: CommandType::CCommand,
                symbol: "".to_string(),
                dest: dest.to_string(),
                comp: comp.to_string(),
                jump: jump.to_string(),
            });
        }

        // Return a type L command object if the current line matches type L command syntax
        let re_l_cmd = Regex::new(r"^\s*\(([\w_\.\$:]+)\)\s*(?://.*)*$").unwrap();
        if re_l_cmd.is_match(current_line) {
            let cap = re_l_cmd.captures(current_line).unwrap();
            let symbol = cap.get(1).unwrap().as_str();
            return Ok(Command {
                cmd_type: CommandType::LCommand,
                symbol: symbol.to_string(),
                dest: "".to_string(),
                comp: "".to_string(),
                jump: "".to_string(),
            });
        }

        panic!("Invalid command: {}", current_line);
    }
}
