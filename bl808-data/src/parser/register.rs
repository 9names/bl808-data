use crate::parser::Field;
use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub address_offset: String,
    pub fields: Vec<Field>,
    pub reset_value: Option<String>,
}

impl Register {
    pub fn new() -> Register {
        Register {
            name: "".to_string(),
            description: "".to_string(),
            address_offset: "".to_string(),
            fields: vec![],
            reset_value: None,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Assuming u32 register sizes for now
        // Calculate reset value
        let mut reset_value: u32 = 0;
        for field in &self.fields {
            if let Some(reset_value_str) = field.reset_value.strip_prefix("0x") {
                let value = u32::from_str_radix(reset_value_str, 16).unwrap();
                let offset = field.lsb.parse::<u32>().unwrap();
                reset_value += value << offset;
            }
        }
        write!(f, "<register>\n<name>{}</name>\n<description>{}</description>\n<addressOffset>{}</addressOffset>\n<resetValue>{:#010X}</resetValue>\n<fields>\n", self.name, self.description, self.address_offset, reset_value)?;
        for field in &self.fields {
            write!(f, "{}", field)?;
        }
        write!(f, "</fields>\n</register>",)
    }
}
