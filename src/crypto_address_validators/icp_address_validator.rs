pub fn icp_address_validator(address: &str) -> bool {
    for c in address.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_icp_address_validator() {
        assert_eq!(
            icp_address_validator(
                "1586c7c0eab0b057126a863510f050d7138ec14cf6709775bcf220df617e61fa"
            ),
            true
        );
    }
}
