use bl808_data::parser::Parser;
use bl808_data::svd_fragments_bl616 as svd_fragments;
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
        .join("bl616")
        .join("std")
        .join("include")
        .join("hardware");
    print!("{}", svd_fragments::HEADER);

    let _ = peripheral(&sdk_path.join("aon_reg.h"), svd_fragments::AON);
    let _ = peripheral(&sdk_path.join("cci_reg.h"), svd_fragments::CCI);

    let _ = peripherals(
        &[
            &sdk_path.join("ef_ctrl_reg.h"),
            &sdk_path.join("ef_data_reg.h"),
        ],
        svd_fragments::EF_DATA,
    );
    let _ = peripheral(&sdk_path.join("glb_reg.h"), svd_fragments::GLB);
    //let _ = peripheral("sources/headers_bl616/bl_mcu_sdk/gpip_reg.h", svd_fragments::GPIP);
    let _ = peripheral(&sdk_path.join("hbn_reg.h"), svd_fragments::HBN);
    //let _ = peripheral("sources/headers_bl616/bl_mcu_sdk/ipc_reg.h", svd_fragments::IPC0);
    let _ = peripheral(&sdk_path.join("mcu_misc_reg.h"), svd_fragments::MCU_MISC);

    // // let _ = peripheral("sources/headers/bl_mcu_sdk/mm_glb_reg.h", svd_fragments::GLB);
    // // let _ = peripheral("sources/headers/bl_mcu_sdk/mm_misc_reg.h", svd_fragments::);

    let _ = peripheral(&sdk_path.join("pds_reg.h"), svd_fragments::PDS);
    // let _ = peripherals(
    //     &[
    //         "sources/headers_bl616/bl_mcu_sdk/psram_reg.h",
    //         "sources/headers_bl616/bl_mcu_sdk/psram_uhs_reg.h",
    //     ],
    //     svd_fragments::PSRAM_CTRL,
    // );
    let _ = peripheral(&sdk_path.join("sdh_reg.h"), svd_fragments::SDH);

    let _ = peripheral(&sdk_path.join("sf_ctrl_reg.h"), svd_fragments::SF_CTRL);

    let _ = peripheral(&sdk_path.join("tzc_sec_reg.h"), svd_fragments::TZC_SEC);
    let _ = peripheral(&sdk_path.join("tzc_nsec_reg.h"), svd_fragments::TZC_NSEC);

    println!("{}", svd_fragments::FOOTER);

    Ok(())
}

// Convenience wrapper for calling with a single filename
fn peripheral(filename: &Path, fragment: &str) -> Result<(), std::io::Error> {
    peripherals(&[filename], fragment)
}

fn peripherals(filenames: &[&Path], fragment: &str) -> Result<(), std::io::Error> {
    // Create our parse context
    let mut parser = Parser::new();

    println!("<peripheral>\n{fragment}<registers>\n");

    for filename in filenames {
        let f = std::fs::read(filename)?;
        for (linenum, l) in f.split(|b| b == &b'\n').enumerate() {
            let l = String::from_utf8_lossy(l);
            parser.parse(linenum, l.to_string());
        }
    }

    // Dump out all the registers
    for register in parser.registers() {
        print!("{register}");
    }
    print!("\n</registers>\n</peripheral>");
    Ok(())
}
