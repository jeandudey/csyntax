/// Taken from https://github.com/rust-lang/rust/blob/master/src/libsyntax/str.rs
pub fn char_at(s: &str, byte: usize) -> char {
    s[byte..].chars().next().unwrap()
}
