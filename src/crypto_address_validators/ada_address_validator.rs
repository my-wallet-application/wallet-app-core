use regex::Regex;

pub fn ada_address_validator(address: &str) -> bool {
    is_valid_address_shelley(address) || is_valid_address_byron(address)
}

fn is_valid_address_shelley(address: &str) -> bool {
    todo!("Implement")
}

fn is_valid_address_byron(address: &str) -> bool {
    todo!("Implement")
}
#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let result = super::ada_address_validator(
            "addr1v9vsw9jysw9w5e8snl5jpxtt63eml60csmmr6xvj6xl7kccg33l7m",
        );

        assert!(result)
    }
}
