// aux-build: helper_export.rs --crate-type rlib --edition 2021
// aux-build: private_import.rs --crate-type proc-macro --edition 2021
// compile-flags: --edition 2021
extern crate private_import;
use private_import::derive;
#[derive(arbitrary_tokens!)]
struct Direct;
#[private_import::derive(arbitrary_tokens!)]
struct Qualified;
macro_rules! inherited_textual { () => { 17 }; }
mod builtin {
    #[derive(Debug)]
    pub struct Ordinary;
    pub fn textual_value() -> u32 { inherited_textual!() }
}
fn main() {
    let _ = (Direct, Qualified);
    glob_scope::check();
    assert_eq!(builtin::textual_value(), 17);
    { #[derive(tokens!)] struct Block; let _ = Block; }
    assert_eq!(format!("{:?}", builtin::Ordinary), "Ordinary");
}
mod glob_scope {
    use private_import::*;
    use private_import::derive; // Explicit import disambiguates the builtin prelude.
    #[derive(tokens!)]
    struct Glob;
    pub fn check() { let _ = Glob; }
}
