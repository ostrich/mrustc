// aux-build: escaped.rs --crate-type proc-macro --edition 2021
// compile-flags: --edition 2021
extern crate escaped;
fn main() {
    assert_eq!(escaped::unicode_underscore!().as_bytes(), &[0xf0, 0x9f, 0x98, 0x80]);
}
