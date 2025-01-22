pub fn validate_btc_address(address: &str) -> bool {
    if address.len() == 0 {
        return false;
    }
    if address.starts_with("1") {
        for c in address.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }
    if address.starts_with("3") {
        for c in address.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    if address.starts_with("bc1") {
        for c in address.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let address1 = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        let address2 = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let address3 = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";
        assert_eq!(super::validate_btc_address(address1), true);
        assert_eq!(super::validate_btc_address(address2), true);
        assert_eq!(super::validate_btc_address(address3), true);
    }
}
