pub fn xlm_address_validator(address: &str) -> bool {
    if address.len() != 56 {
        return false;
    }

    if !address.starts_with("G") {
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
    fn test_xlm_address_validator() {
        assert_eq!(
            xlm_address_validator("GC7OHFPWPSWXL4HMN6TXAG54MTZSMJIASWHO6KVRQNHNCXEAHWDSGGC3"),
            true
        );
        assert_eq!(
            xlm_address_validator("GC65CUPW2IMTJJY6CII7F3OBPVG4YGASEPBBLM4V3LBKX62P6LA24OFV"),
            true
        );
    }
}
