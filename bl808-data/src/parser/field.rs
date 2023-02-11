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

impl Default for Field {
    fn default() -> Self {
        Self::new()
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
        "r" => "read-only",
        "R" => "read-only",
        "rw" => "read-write",
        "RW" => "read-write",
        "RW1C" => "read-write", // TODO: needs oneToClear in modifiedWriteValues.
        "RWAC" => "read-write", // TODO: needs anyToClear? in modifiedWriteValues.
        "r/w" => "read-write",
        "RO" => "read-only",
        "ROC" => "read-only", // TODO: needs readToClear? in modifiedWriteValues.
        "w" => "write-only",
        "WO" => "write-only",
        "w1p" => "write-only", // TODO: needs oneTo(something) in modifiedWriteValues.
        "w1c" => "write-only", // TODO: needs oneToClear in modifiedWriteValues.
        "rsvd" => "read-only",
        "RSVD" => "read-only",
        _ => "UNMAPPED_PLZ_FIX",
    }
}

impl Field {
    #[allow(dead_code)]
    pub fn to_svdtools_yaml(&self) -> String {
        let msb = str::parse::<u32>(&self.msb).expect("Couldn't parse Field MSB as an int");
        let lsb = str::parse::<u32>(&self.lsb).expect("Couldn't parse Field LSB as an int");
        let bit_width = (msb + 1) - lsb;

        // If the description field is empty or the same as the name, drop it
        let descriptionfield = if !self.description.trim().is_empty()
            && self.description.trim().to_lowercase() != self.name.trim().to_lowercase()
        {
            format!("      description: {}\n", self.description.trim())
        } else {
            String::from("")
        };
        format!(
        "    {}:\n{descriptionfield}      bitOffset: {}\n      bitWidth: {}\n      access: {}\n",
        self.name.to_ascii_lowercase(),
        self.lsb,
        bit_width,
        svd_access_map(&self.access),
    )
    }

    #[allow(dead_code)]
    pub fn to_svd2rust_style_yaml(&self) -> String {
        // If the description field is empty or the same as the name, drop it
        let descriptionfield = if !self.description.trim().is_empty()
            && self.description.trim().to_lowercase() != self.name.trim().to_lowercase()
        {
            format!("      description: {}\n", self.description.trim())
        } else {
            String::from("")
        };
        format!(
            "    {}:\n{descriptionfield}      bitRange: [{}:{}]\n      access: {}\n",
            self.name.to_ascii_lowercase(),
            self.msb,
            self.lsb,
            svd_access_map(&self.access),
        )
    }

    #[allow(dead_code)]
    pub fn to_xml(&self) -> String {
        // If the description field is empty or the same as the name, drop it
        let descriptionfield = if !self.description.trim().is_empty()
            && self.description.trim().to_lowercase() != self.name.trim().to_lowercase()
        {
            format!("<description>{}</description>\n", self.description.trim())
        } else {
            String::from("")
        };
        format!(
            "<field>\n<name>{}</name>\n{descriptionfield}<bitRange>[{}:{}]</bitRange>\n<access>{}</access>\n</field>\n",
            self.name.to_ascii_lowercase(),
            self.msb,
            self.lsb,
            svd_access_map(&self.access)
        )
    }
}
