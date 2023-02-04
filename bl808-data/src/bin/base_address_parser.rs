use bl808_data::parser::peripheral::parse_peri_address;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let f = std::fs::read("sources/headers/bl_mcu_sdk/bl808.h")?;
    for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
        let l = String::from_utf8_lossy(l);
        let address = parse_peri_address(l.to_string(), linenum);
        if let Some(peri) = address {
            print!("{peri}");
        }
    }

    Ok(())
}
