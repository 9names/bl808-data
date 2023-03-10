use bl808_data::parser::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let f = std::fs::read("sources/testing/test_registers.h")?;

    // Create our parse context
    let mut parser = Parser::new();

    // Run all the input through the parser
    for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
        let l = String::from_utf8_lossy(l);
        parser.parse(linenum, l.to_string());
    }

    // Dump out all the registers
    for register in parser.registers() {
        println!("{register}");
    }
    Ok(())
}
