pub fn dot_address_validator(address: &str) -> bool {
    if !address.starts_with("1") {
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
    #[test]
    fn test() {
        let result =
            super::dot_address_validator("12vZUy8xkvrDMev62QhLZ3kDYRtVKoUhLo8bvEZaxuhKnMyk");
        assert!(result)
    }
}
