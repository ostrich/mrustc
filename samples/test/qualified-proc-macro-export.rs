// aux-build: named_derive.rs --crate-type proc-macro --edition 2021
// compile-flags: --edition 2021
extern crate named_derive;
#[named_derive::derive(arbitrary_tokens!)]
struct Qualified;
fn main() { let _ = Qualified; }
