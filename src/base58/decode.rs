const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn decode(src: &str) -> Option<Vec<u8>> {
    base_x::decode(BASE58_ALPHABET, src).ok()
}
