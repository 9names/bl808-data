use tracing::{event, instrument, Level};
pub mod parser;
pub mod svd_fragments;
pub mod svd_fragments_bl616;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseState {
    /// Looking for start of Peripheral block: "struct glb_reg {"
    PeripheralStart,
    /// Looking for register address: "/* 0x0 : soc_info0 */""
    RegAddress,
    /// Looking for start of union: "union {"
    UnionStart,
    /// Looking for start of struct: "struct {"
    StructStart,
    /// Field: "uint32_t reserved_0_26 : 27; /* [26: 0],       rsvd,        0x0 */"
    FieldEntry,
    // End of struct (in case we missed it):  "} BF;"
    EndOfStruct,
    /// Looking for 2nd union member: "uint32_t WORD;"
    Size,
    /// Looking for name of the union: "} soc_info0;"
    Name,
}

#[derive(Debug)]
pub enum ParseResult {
    Match(Vec<String>),
    Capture(Vec<String>),
}

#[macro_export]
macro_rules! regex {
    ($re:literal) => {{
        ::ref_thread_local::ref_thread_local! {
            static managed REGEX: ::regex::Regex = ::regex::Regex::new($re).unwrap();
        }
        <REGEX as ::ref_thread_local::RefThreadLocal<::regex::Regex>>::borrow(&REGEX)
    }};
}

#[instrument]
pub fn parseit(
    state: ParseState,
    line: String,
    linenum: usize,
) -> (ParseState, Option<ParseResult>) {
    let mut state = state;
    let mut data: Vec<String> = vec![];
    match state {
        // Looking for start of register block: "struct glb_reg {"
        ParseState::PeripheralStart => {
            if let Some(m) = regex!(r"\s*struct\s*([a-zA-Z_\d]*)\s*\{").captures(&line) {
                state = ParseState::RegAddress;
                // 1st capture is the name of the register block
                data.push(String::from(m.get(1).unwrap().as_str()));
                event!(Level::TRACE, "\nCaptures: {}", data[0]);
                (state, Some(ParseResult::Capture(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for register address: "/* 0x0 : soc_info0 */""
        ParseState::RegAddress => {
            if let Some(m) = regex!(r"};").captures(&line) {
                state = ParseState::PeripheralStart;
                (state, None)
            } else if let Some(m) = regex!(r"\s*\.*/* (0x[a-fA-F_\d]*) : (.*) \*/").captures(&line) {
                state = ParseState::UnionStart;
                // 1st capture is register offset
                data.push(String::from(m.get(1).unwrap().as_str()));
                // 2nd capture is register name
                data.push(String::from(m.get(2).unwrap().as_str()));
                event!(Level::TRACE, "\nCaptures: {}, {}", data[0], data[1]);
                (state, Some(ParseResult::Capture(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for start of union: "union {"
        ParseState::UnionStart => {
            if let Some(m) = regex!(r"\s*union\s*\{").captures(&line) {
                state = ParseState::StructStart;
                data.push(String::from(m.get(0).unwrap().as_str()));
                event!(Level::TRACE, "\nMatch: {}", data[0]);
                (state, Some(ParseResult::Match(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for start of struct: "struct {"
        // Being a bit more permissive here to allow for sdh_reg, which puts the open brace on a new line
        ParseState::StructStart => {
            if let Some(m) = regex!(r"\s*struct\s*").captures(&line) {
                state = ParseState::FieldEntry;
                data.push(String::from(m.get(0).unwrap().as_str()));
                event!(Level::TRACE, "\nMatch: {}", data[0]);
                (state, Some(ParseResult::Match(data)))
            }
            // empty brace on line. maybe should be it's own state?
            else if let Some(_) = regex!(r"\w*(\{)\w*").captures(&line) {
                state = ParseState::FieldEntry;
                (state, None)
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for field or end of struct:
        // Field: "uint32_t reserved_0_26 : 27; /* [26: 0],       rsvd,        0x0 */"
        // End of struct:  "} BF;"
        ParseState::FieldEntry => {
            if let Some(m) = regex!(r"\s*uint\d\d?\d?_t *([a-zA-Z_\d]*) *: *(\d*); */\* *\[([\d: ]*)\],\s*([\S]*?)\s*,\s*(0x[\da-fA-F]*) \*/.*").captures(&line) {
                state = ParseState::FieldEntry;
                // 1st capture is the field name
                data.push(String::from(m.get(1).unwrap().as_str()));
                // 2nd capture is field width
                data.push(String::from(m.get(2).unwrap().as_str()));
                // 3rd capture is the range of bits
                data.push(String::from(m.get(3).unwrap().as_str()));
                // 4th capture is access modifier
                data.push(String::from(m.get(4).unwrap().as_str()));
                // 5th capture is reset value
                data.push(String::from(m.get(5).unwrap().as_str()));
                event!(Level::TRACE,"\nCaptures: {}, {}, {}, {}, {}", data[0], data[1], data[2], data[3], data[4]);
                (state, Some(ParseResult::Capture(data)))
            }
            else {
                event!(Level::TRACE,"BF capture, moving to field parsing");
                if let Some(m) = regex!(r"\s*} *BF;").captures(&line) {
                    state = ParseState::Size;
                    data.push(String::from(m.get(0).unwrap().as_str()));
                    event!(Level::TRACE,"\nMatch: {}", data[0]);
                    (state, Some(ParseResult::Match(data)))
                } else {
                    event!(Level::TRACE, "\nMode {state:?} unhandled line {linenum}:{line}");
                    event!(Level::TRACE,"No Capture, moving to field parsing");
                    state = ParseState::EndOfStruct;
                    (state, None)
                }
            }
        }
        // End of struct (in case we missed it):  "} BF;"
        ParseState::EndOfStruct => {
            if let Some(m) = regex!(r"\s*} *BF;").captures(&line) {
                state = ParseState::Size;
                data.push(String::from(m.get(0).unwrap().as_str()));
                event!(Level::TRACE, "\nMatch: {}", data[0]);
                (state, Some(ParseResult::Match(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for 2nd union member: "uint32_t WORD;"
        ParseState::Size => {
            if let Some(m) = regex!(r"\s*(uint32_t)\s*WORD;").captures(&line) {
                state = ParseState::Name;
                data.push(String::from(m.get(0).unwrap().as_str()));
                event!(Level::TRACE, "\nCaptures: {}", data[0]);
                (state, Some(ParseResult::Capture(data)))
            } else if let Some(m) = regex!(r"\s*(uint16_t)\s*SHORT;").captures(&line) {
                state = ParseState::Name;
                data.push(String::from(m.get(0).unwrap().as_str()));
                event!(Level::TRACE, "\nCaptures: {}", data[0]);
                (state, Some(ParseResult::Capture(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
        // Looking for name of the union: "} soc_info0;"
        ParseState::Name => {
            if let Some(m) = regex!(r"\s*}\s*([a-zA-Z_\-\d]*);").captures(&line) {
                state = ParseState::RegAddress;
                data.push(String::from(m.get(1).unwrap().as_str()));
                event!(Level::TRACE, "\nCaptures: {}", data[0]);
                (state, Some(ParseResult::Capture(data)))
            } else {
                event!(
                    Level::TRACE,
                    "\nMode {state:?} unhandled line {linenum}:{line}"
                );
                (state, None)
            }
        }
    }
}
