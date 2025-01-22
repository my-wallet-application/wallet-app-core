pub fn sol_address_validator(address: &str) -> bool {
    if address.len() < 32 {
        return false;
    }

    if address.len() > 64 {
        return false;
    }

    for c in address.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_sol_address_validator() {
        assert_eq!(
            sol_address_validator("7EcDhSYGxXyscszYEp35KHN8vvw3svAuLKTzXwCFLtV"),
            true
        );

        assert_eq!(
            sol_address_validator("DpNXPNWvWoHaZ9P3WtfGCb2ZdLihW8VW1w1Ph4KDH9iG"),
            true
        );
    }
}
