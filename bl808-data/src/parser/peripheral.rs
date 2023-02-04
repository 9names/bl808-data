use crate::regex;
use core::fmt;
use tracing::{event, instrument, Level};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peripheral {
    pub name: String,
    pub address: String,
}

impl Peripheral {
    pub fn new() -> Peripheral {
        Peripheral {
            name: "".to_string(),
            address: "".to_string(),
        }
    }

    pub fn to_yaml(&self) -> String {
        format!("{}:\n  address: {}\n", self.name.to_ascii_lowercase(), self.address.to_ascii_lowercase(),)
    }
}

impl fmt::Display for Peripheral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_yaml())
    }
}

#[instrument]
pub fn parse_peri_address(line: String, linenum: usize) -> Option<Peripheral> {
    if let Some(m) = regex!(r"#define (\w+)_BASE\W*\(\(uint32_t\)(0x\d+)\)").captures(&line) {
        // We've captured a peripheral base address
        let name = String::from(m.get(1).unwrap().as_str());
        let address = String::from(m.get(2).unwrap().as_str());
        event!(Level::TRACE, "\nCaptures: {name} {address}");
        Some(Peripheral { name, address })
    } else {
        event!(
            Level::TRACE,
            "\nPeripheral parser skipping line {linenum}:{line}"
        );
        None
    }
}
