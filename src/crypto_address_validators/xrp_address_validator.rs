pub fn xrp_address_validator(address: &str) -> bool {
    if !address.starts_with('r') {
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
    fn test_xrp_address_validator() {
        assert_eq!(
            xrp_address_validator("rG1QQv2nh2gr7RCZ1P8YYcBUKCCN633jCn"),
            true
        );
    }
}
