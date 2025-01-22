pub fn validate_hbar_address(address: &str) -> bool {
    if address.contains('.') {
        // Account ID format
        let parts: Vec<&str> = address.split('.').collect();
        if parts.len() == 3 {
            return parts.iter().all(|part| part.parse::<u64>().is_ok()) && parts[2] != "0";
        }
    } else if address.starts_with("0x") {
        // Alias format
        return address.chars().skip(2).all(|c| c.is_ascii_hexdigit());
    }
    false
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let address1 = "0.0.123456";
        let address2 = "0x302a300506032b6570032100a1b2c3d4e5f67890abcdef1234567890abcdef1234567890abcdef123456";

        assert!(super::validate_hbar_address(address1));
        assert!(super::validate_hbar_address(address2));
    }
}
