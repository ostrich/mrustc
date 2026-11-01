// aux-build: escaped.rs --crate-type proc-macro --edition 2021
// compile-flags: --edition 2021
extern crate escaped;
fn main() {
    let (text, bytes) = escaped::escaped!();
    assert_eq!(text.as_bytes(), b"a\n\t\"\\A\xf0\x9f\x98\x80");
    assert_eq!(bytes, &[0, 255, 9]);
    println!("procedural macro escaped strings pass");
}
