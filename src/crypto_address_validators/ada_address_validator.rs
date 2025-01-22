use regex::Regex;

pub fn ada_address_validator(address: &str) -> bool {
    let shelley_regex = Regex::new(r"^addr[a-z0-9]{58,}$").unwrap();

    // Cardano Byron address regex (base58 format)
    let byron_regex = Regex::new(r"^[A-HJ-NP-Za-km-z1-9]{98,}$").unwrap();

    // Validate against Shelley or Byron regex
    shelley_regex.is_match(address) || byron_regex.is_match(address)
}
