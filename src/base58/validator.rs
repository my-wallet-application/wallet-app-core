pub fn is_base_58(src: &str) -> bool {
    let mut count = 0;
    for c in src.chars() {
        if c.is_ascii_alphanumeric() {
            count += 1;
        }
    }
    count == src.len()
}
