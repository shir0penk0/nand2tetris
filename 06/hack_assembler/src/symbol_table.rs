use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct SymbolTable {
    symbol_map: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbol_map: HashMap::from([
                                      ("SP".to_string(), 0),
                                      ("LCL".to_string(), 1),
                                      ("ARG".to_string(), 2),
                                      ("THIS".to_string(), 3),
                                      ("THAT".to_string(), 4),
                                      ("R0".to_string(), 0),
                                      ("R1".to_string(), 1),
                                      ("R2".to_string(), 2),
                                      ("R3".to_string(), 3),
                                      ("R4".to_string(), 4),
                                      ("R5".to_string(), 5),
                                      ("R6".to_string(), 6),
                                      ("R7".to_string(), 7),
                                      ("R8".to_string(), 8),
                                      ("R9".to_string(), 9),
                                      ("R10".to_string(), 10),
                                      ("R11".to_string(), 11),
                                      ("R12".to_string(), 12),
                                      ("R13".to_string(), 13),
                                      ("R14".to_string(), 14),
                                      ("R15".to_string(), 15),
                                      ("SCREEN".to_string(), 16384),
                                      ("KBD".to_string(), 24576),
            ]),
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u32) {
        self.symbol_map.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &String) -> bool {
        self.symbol_map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &String) -> Option<&u32> {
        self.symbol_map.get(symbol)
    }
}
