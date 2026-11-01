// aux-build: named_derive.rs --crate-type proc-macro --edition 2021
// compile-flags: --edition 2021
extern crate named_derive;
use named_derive::derive;
#[derive(arbitrary_tokens!, not_builtin_paths!)]
struct Example;
fn main() { let _ = Example; println!("named derive attribute shadows builtin"); }
