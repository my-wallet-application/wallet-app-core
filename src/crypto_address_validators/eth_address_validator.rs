pub fn eth_address_validator(address: &str) -> bool {
    if address.len() != 42 {
        return false;
    }
    if !address.starts_with("0x") {
        return false;
    }
    for c in address.chars().skip(2) {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}
