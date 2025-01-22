pub fn avax_address_validator(address: &str) -> bool {
    if !address.starts_with("0x") || address.len() != 42 {
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

    use super::*;

    #[test]
    fn test_avax_address_validator() {
        assert_eq!(
            avax_address_validator("0x09383137c1eee3e1a8bc781228e4199f6b4a9bbf"),
            true
        );

        assert_eq!(
            avax_address_validator("0x95f0895C76967fE8A3c1dA3c9b53e8bb31C91073"),
            true
        );
    }
}
