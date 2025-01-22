pub fn hbar_address_validator(address: &str) -> bool {
    let regex = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();

    if !regex.is_match(address) {
        return false;
    }

    // Split the address into shard, realm, and account parts
    let parts: Vec<&str> = address.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    // Validate each part is a non-negative integer
    parts.iter().all(|part| part.parse::<u64>().is_ok())
}
