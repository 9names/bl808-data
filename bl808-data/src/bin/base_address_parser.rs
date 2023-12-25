use bl808_data::parser::peripheral::parse_peri_address;
use std::{fs::File, io::Write, path::Path};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_max_level(Level::INFO) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let chips = ["bl602", "bl616", "bl702", "bl702l", "bl808"];

    for chip in chips {
        parse_base(chip);
    }

    Ok(())
}

fn parse_base(chip: &str) -> anyhow::Result<()> {
    let sdk_path = Path::new("sources")
        .join("bouffalo_sdk")
        .join(chip)
        .join("std")
        .join("include")
        .join("hardware");
    let f = std::fs::read(sdk_path.join(format!("{chip}.h")))?;
    let path = Path::new("generated_toplevel");
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }
    let mut output = File::create(path.join(format!("{chip}.yaml")))?;

    for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
        let l = String::from_utf8_lossy(l);
        let address = parse_peri_address(l.to_string(), linenum);
        if let Some(peri) = address {
            output.write_all(format!("{peri}").as_bytes()).unwrap();
        }
    }
    Ok(())
}
