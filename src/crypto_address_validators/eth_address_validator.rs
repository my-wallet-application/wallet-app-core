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

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let result = super::eth_address_validator("0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae");
        assert!(result)
    }
}
