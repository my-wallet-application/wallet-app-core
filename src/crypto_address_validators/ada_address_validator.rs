pub fn ada_address_validator(address: &str) -> bool {
    if address.starts_with("addr1") {
        if address.len() < 10 {
            return false;
        }

        for c in address.chars().skip(5) {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }

        return true;
    }

    if address.starts_with("stake1") {
        if address.len() < 10 {
            return false;
        }

        for c in address.chars().skip(6) {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    if address.starts_with("Ae2") {
        if address.len() < 10 {
            return false;
        }

        for c in address.chars().skip(3) {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    if address.starts_with("Dd") {
        if address.len() < 10 {
            return false;
        }

        for c in address.chars().skip(2) {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }
        return true;
    }

    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let result = super::ada_address_validator(
            "addr1qx2fxv2e0vtpuzjhkt3j22k9m90gy3pkjd5lg4cqpf5g2rg69gf4grm8t3w5r0jh2wt3kc4e9vg06ue2gxv0j7nx4l2snzvkg8",
        );

        assert!(result)
    }
}
