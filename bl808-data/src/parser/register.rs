use crate::parser::Field;
use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub description: String,
    pub address_offset: String,
    pub fields: Vec<Field>,
    pub reset_value: Option<String>,
    pub size: Option<u8>,
}

impl Register {
    pub fn new() -> Register {
        Register {
            name: "".to_string(),
            description: "".to_string(),
            address_offset: "".to_string(),
            fields: vec![],
            reset_value: None,
            size: None,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_xml())
    }
}

impl Register {
    fn reset_value(&self) -> u32 {
        // Assuming u32 register sizes for now
        let mut reset_value: u32 = 0;
        for field in &self.fields {
            if let Some(reset_value_str) = field.reset_value.strip_prefix("0x") {
                let value = u32::from_str_radix(reset_value_str, 16).unwrap();
                let offset = field.lsb.parse::<u32>().unwrap();
                reset_value += value << offset;
            }
        }
        reset_value
    }

    pub fn to_yaml(&self) -> String {
        let sizefield = if self.size.is_some() {
            let size = self.size.unwrap();
            // default size is 32, no need to print it
            if size != 32 {
                format!("  size: {}\n", size)
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };
        let descriptionfield = if !self.description.trim().is_empty() {
            format!("  description: {}\n", self.description.trim())
        } else {
            String::from("")
        };
        let mut out = format!(
            "{}:\n{descriptionfield}  addressOffset: {}\n{sizefield}  resetValue: {:#010X}\n  fields:\n",
            self.name.to_ascii_lowercase(),
            self.address_offset,
            self.reset_value()
        );
        for field in &self.fields {
            out += &field.to_svdtools_yaml();
        }
        out
    }

    pub fn to_xml(&self) -> String {
        let sizefield = if self.size.is_some() {
            let size = self.size.unwrap();
            // default size is 32, no need to print it
            if size != 32 {
                format!("<size>{}</size>\n", size)
            } else {
                String::from("")
            }
        } else {
            String::from("")
        };
        let descriptionfield = if !self.description.trim().is_empty() {
            format!("<description>{}</description>\n", self.description.trim())
        } else {
            String::from("")
        };
        let mut out = format!(
            "<register>\n<name>{}</name>\n{descriptionfield}<addressOffset>{}</addressOffset>\n{sizefield}<resetValue>{:#010X}</resetValue>\n<fields>\n",
            self.name.to_ascii_lowercase(),
            self.address_offset,
            self.reset_value()
        );
        for field in &self.fields {
            out += &format!("{}", field);
        }
        out += "</fields>\n</register>";
        out
    }
}
