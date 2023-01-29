use crate::{parseit, ParseState};

mod field;
mod register;
pub use field::svd_access_map;
pub use field::Field;
pub use register::Register;

#[derive(Debug)]
pub struct Parser {
    state: ParseState,
    register: Option<Register>,
    registers: Vec<Register>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            state: ParseState::PeripheralStart,
            register: None,
            registers: vec![],
        }
    }

    pub fn parse(&mut self, line_num: usize, line: String) {
        let (newstate, parse_result) = parseit(self.state, line, line_num);
        // Match against the state *before* we parsed (we might have transitioned due to parseit)
        match self.state {
            ParseState::PeripheralStart => {} // Don't do anything with peri
            ParseState::RegAddress => {
                if let Some(parse) = parse_result {
                    if let Some(reg) = &self.register {
                        self.registers.push(reg.clone());
                    }
                    let mut reg = Register::new();
                    match parse {
                        crate::ParseResult::Match(_) => panic!("Not expecting match"),
                        crate::ParseResult::Capture(c) => {
                            reg.address_offset = c[0].clone();
                            reg.description = c[1].clone();
                        }
                    }
                    self.register = Some(reg);
                }
            }
            ParseState::UnionStart => {}
            ParseState::StructStart => {}
            ParseState::StructStart2 => {}
            ParseState::FieldEntry => {
                if let Some(parse) = parse_result {
                    let mut field = Field::new();
                    match parse {
                        crate::ParseResult::Match(_m) => {
                            // println!("what a match: {m:?}")
                        }
                        crate::ParseResult::Capture(c) => {
                            // [0] is the field name
                            // [1] is field width
                            // [2] is range of bits
                            // [3] is access modifier
                            // [4] is reset value

                            field.name = c[0].trim().to_string();
                            // c[1] is number of bits, we don't need that.
                            if c[2].contains(':') {
                                let mut c_arr = c[2].split(':');
                                let msb = c_arr.next();
                                let lsb = c_arr.next();
                                field.msb = msb.unwrap().trim().to_string();
                                field.lsb = lsb.unwrap().trim().to_string();
                            } else {
                                field.msb = c[2].trim().to_string();
                                field.lsb = field.msb.clone();
                            }
                            field.access = c[3].trim().to_string();
                            field.reset_value = c[4].trim().to_string();
                        }
                    }
                    if let Some(reg) = self.register.as_mut() {
                        // TODO: don't generate empty field so we don't have to skip it
                        if !field.name.trim().is_empty() {
                            reg.fields.push(field);
                        }
                    }
                }
            }
            ParseState::EndOfStruct => {}
            ParseState::Size => {}
            ParseState::Name => {
                if let Some(parse) = parse_result {
                    match parse {
                        crate::ParseResult::Match(_n) => {
                            // println!("whats my name? {n:?}")
                        }
                        crate::ParseResult::Capture(c) => {
                            if let Some(mut reg) = self.register.take() {
                                reg.name = c[0].clone();
                                // At this point our register is complete, push it.
                                self.registers.push(reg);
                            }
                        }
                    }
                }
            }
        }
        self.state = newstate
    }
    pub fn registers(&self) -> Vec<Register> {
        self.registers.clone()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
