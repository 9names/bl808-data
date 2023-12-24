use bl808_data::parser::peripheral::parse_peri_address;
use std::path::Path;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_max_level(Level::INFO) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let sdk_path = Path::new("sources")
        .join("bouffalo_sdk")
        .join("bl808")
        .join("std")
        .join("include")
        .join("hardware");
    let f = std::fs::read(sdk_path.join("bl808.h"))?;
    for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
        let l = String::from_utf8_lossy(l);
        let address = parse_peri_address(l.to_string(), linenum);
        if let Some(peri) = address {
            print!("{peri}");
        }
    }

    Ok(())
}
