use std::{io::Write, path::Path};

use bl808_data::parser::{peripheral::parse_peri_address, Parser};
// use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> anyhow::Result<()> {
    // Use tracing to get good debug tracing, and register stdout as a tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        //.with_max_level(Level::INFO) // Set this to DEBUG or TRACE to get debugging info
        .with_writer(std::io::stderr) // Write to stderr so we can still pipe output to file
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let filenames = [
        "sources/headers/bl_mcu_sdk/aon_reg.h",
        "sources/headers/bl_mcu_sdk/cci_reg.h",
        "sources/headers/bl_mcu_sdk/codec_misc_reg.h",
        "sources/headers/bl_mcu_sdk/ef_ctrl_reg.h",
        "sources/headers/bl_mcu_sdk/ef_data_0_reg.h",
        "sources/headers/bl_mcu_sdk/ef_data_1_reg.h",
        "sources/headers/bl_mcu_sdk/glb_reg.h",
        "sources/headers/bl_mcu_sdk/gpip_reg.h",
        "sources/headers/bl_mcu_sdk/hbn_reg.h",
        "sources/headers/bl_mcu_sdk/ipc_reg.h",
        "sources/headers/bl_mcu_sdk/mcu_misc_reg.h",
        "sources/headers/bl_mcu_sdk/mm_glb_reg.h",
        "sources/headers/bl_mcu_sdk/mm_misc_reg.h",
        "sources/headers/bl_mcu_sdk/pds_reg.h",
        "sources/headers/bl_mcu_sdk/psram_reg.h",
        "sources/headers/bl_mcu_sdk/psram_uhs_reg.h",
        "sources/headers/bl_mcu_sdk/sdh_reg.h",
        "sources/headers/bl_mcu_sdk/sf_ctrl_reg.h",
        "sources/headers/bl_mcu_sdk/tzc_sec_reg.h",
        "sources/headers/bl_mcu_sdk/tzc_nsec_reg.h",
    ];

    let chip_filename = "sources/headers/bl_mcu_sdk/bl808.h";

    let _ = peripherals(&filenames);

    let _ = peripheral_base(chip_filename);

    Ok(())
}

fn peripherals(filenames: &[&str]) -> Result<(), std::io::Error> {
    // Create generated_yaml dir if it doesn't already exist
    let output_dir = Path::new("generated_yaml");
    std::fs::create_dir_all(output_dir).expect("Unable to create yaml output dir");

    for filename in filenames {
        let path = Path::new(filename);
        let f = std::fs::read(filename)?;
        let mut parser = Parser::new();
        for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
            let l = String::from_utf8_lossy(l);
            parser.parse(linenum, l.to_string());
        }
        // Dump out all the registers to a file
        let target_filename = path.file_stem().expect("Filename missing?");
        let mut target_yaml = output_dir.join(target_filename);
        target_yaml.set_extension("yaml");
        println!("{target_yaml:?}");
        let mut file = std::fs::File::create(&target_yaml).expect("Couldn't create yaml file");
        for register in parser.registers() {
            file.write_all(register.to_yaml().as_bytes())
                .expect("Failed to write yaml to file");
        }
    }

    Ok(())
}

fn peripheral_base(filename: &str) -> Result<(), std::io::Error> {
    let output_dir = Path::new("generated_yaml");
    std::fs::create_dir_all(output_dir).expect("Unable to create yaml output dir");
    let f = std::fs::read(filename)?;
    let target_yaml = output_dir.join("peripheral_base_address.yaml");
    let mut file = std::fs::File::create(&target_yaml).expect("Couldn't create yaml file");
    for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
        let l = String::from_utf8_lossy(l);
        let address = parse_peri_address(l.to_string(), linenum);
        if let Some(peri) = address {
            file.write_all(peri.to_yaml().as_bytes())
                .expect("Failed to write yaml to file");
        }
    }

    Ok(())
}
