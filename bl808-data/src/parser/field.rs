use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub lsb: String,
    pub msb: String,
    pub access: String,
    pub reset_value: String,
}

impl Field {
    pub fn new() -> Field {
        Field {
            name: "".to_string(),
            description: "".to_string(),
            lsb: "".to_string(),
            msb: "".to_string(),
            access: "".to_string(),
            reset_value: "".to_string(),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_xml(),)
    }
}

/// SVD uses different strings to represent access modifiers than what is used in the SDK headers.
/// map these through to SVD versions
pub fn svd_access_map(access: &str) -> &str {
    match access {
        "r/w" => "read-write",
        "rw" => "read-write",
        "r" => "read-only",
        "w" => "write-only",
        "w1p" => "write-only", // TODO: needs oneTo(something) in modifiedWriteValues.
        "w1c" => "write-only", // TODO: needs oneToClear in modifiedWriteValues.
        "rsvd" => "read-only",
        _ => "UNMAPPED_PLZ_FIX",
    }
}

impl Field {
    #[allow(dead_code)]
    pub fn to_svdtools_yaml(&self) -> String {
        let msb = str::parse::<u32>(&self.msb).expect("Couldn't parse Field MSB as an int");
        let lsb = str::parse::<u32>(&self.lsb).expect("Couldn't parse Field LSB as an int");
        let bit_width = (msb + 1) - lsb;

        format!(
        "    {}:\n      description: {}\n      bitOffset: {}\n      bitWidth: {}\n      access: {}\n",
        self.name,
        self.description,
        self.lsb,
        bit_width,
        svd_access_map(&self.access),
    )
    }

    #[allow(dead_code)]
    pub fn to_svd2rust_style_yaml(&self) -> String {
        format!(
            "    {}:\n      description: {}\n      bitRange: [{}:{}]\n      access: {}\n",
            self.name,
            self.description,
            self.msb,
            self.lsb,
            svd_access_map(&self.access),
        )
    }

    #[allow(dead_code)]
    pub fn to_xml(&self) -> String {
        format!(
            "<field>\n<name>{}</name>\n<description>{}</description>\n<bitRange>[{}:{}]</bitRange>\n<access>{}</access>\n</field>\n",
            self.name,
            self.description,
            self.msb,
            self.lsb,
            svd_access_map(&self.access)
        )
    }
}
