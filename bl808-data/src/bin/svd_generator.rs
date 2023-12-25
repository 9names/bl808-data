use bl808_data::parser::Parser;
use bl808_data::svd_fragments;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_max_level(Level::INFO) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let chip = "bl808";
    let sdk_path = Path::new("sources")
        .join("bouffalo_sdk")
        .join(chip)
        .join("std")
        .join("include")
        .join("hardware");
    let path = Path::new("generated");
    if !path.exists() {
        std::fs::create_dir(path)?;
    }
    let peri_list = [
        (vec!["codec_misc_reg.h"], svd_fragments::CODEC),
        (vec!["aon_reg.h"], svd_fragments::AON),
        (vec!["cci_reg.h"], svd_fragments::CCI),
        (
            vec!["ef_ctrl_reg.h", "ef_data_0_reg.h", "ef_data_1_reg.h"],
            svd_fragments::EF_DATA,
        ),
        (vec!["glb_reg.h"], svd_fragments::GLB),
        (vec!["gpip_reg.h"], svd_fragments::GPIP),
        (vec!["hbn_reg.h"], svd_fragments::HBN),
        (vec!["ipc_reg.h"], svd_fragments::IPC0),
        (vec!["mcu_misc_reg.h"], svd_fragments::MCU_MISC),
        (vec!["pds_reg.h"], svd_fragments::PDS),
        (
            vec!["psram_reg.h", "psram_uhs_reg.h"],
            svd_fragments::PSRAM_CTRL,
        ),
        (vec!["sdh_reg.h"], svd_fragments::SDH),
        (vec!["sf_ctrl_reg.h"], svd_fragments::SF_CTRL),
        (vec!["tzc_sec_reg.h"], svd_fragments::TZC_SEC),
        (vec!["tzc_nsec_reg.h"], svd_fragments::TZC_NSEC),
    ];

    let chip = chip.to_uppercase();
    let mut output = File::create(path.join(format!("{chip}.svd")))?;
    output.write_all(svd_fragments::HEADER.as_bytes())?;

    for peri in peri_list {
        let mut paths = Vec::new();
        for file in peri.0 {
            paths.push(sdk_path.join(file));
        }
        let p = peripherals(&paths, peri.1)?;
        output.write_all(p.as_ref())?;
    }

    output.write_all(svd_fragments::FOOTER.as_bytes())?;

    Ok(())
}

fn peripherals(filenames: &[PathBuf], fragment: &str) -> anyhow::Result<String> {
    // Create our parse context
    let mut parser = Parser::new();
    let mut output = String::from_str(&format!("<peripheral>\n{fragment}<registers>\n"))?;

    for filename in filenames {
        let f = std::fs::read(filename)?;
        for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
            let l = String::from_utf8_lossy(l);
            parser.parse(linenum, l.to_string());
        }
    }

    // Dump out all the registers
    for register in parser.registers() {
        output.push_str(register.to_string().as_str());
    }
    output.push_str("\n</registers>\n</peripheral>");
    Ok(output)
}
