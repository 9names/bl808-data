use crate::regex;
use tracing::{event, instrument, Level};

#[instrument]
pub fn parse_peri_address(line: String, linenum: usize) -> Option<Vec<String>> {
    let mut data: Vec<String> = vec![];
    if let Some(m) = regex!(r"#define (\w+)_BASE\W*\(\(uint32_t\)(0x\d+)\)").captures(&line) {
        // We've captured a peripheral base address
        // First capture group is peripheral name
        data.push(String::from(m.get(1).unwrap().as_str()));
        // Second capture group is peripheral address
        data.push(String::from(m.get(2).unwrap().as_str()));

        event!(Level::TRACE, "\nCaptures: {} {}", data[0], data[1]);
        Some(data)
    } else {
        event!(
            Level::TRACE,
            "\nPeripheral parser skipping line {linenum}:{line}"
        );
        None
    }
}
