pub fn trx_address_validator(address: &str) -> bool {
    if !address.starts_with('T') || address.len() != 34 {
        return false;
    }

    for c in address.chars() {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_trx_address_validator() {
        assert_eq!(
            trx_address_validator("TPzPryjrVHNekQveYK8go7kB6K4qVbgwKo"),
            true
        );
    }
}
