// aux-build: opaque_trait.rs --crate-type rlib --edition 2021
// compile-flags: --edition 2021
extern crate opaque_trait;
use opaque_trait::Values;
struct Source;
impl Values for Source {
    fn values() -> std::array::IntoIter<u8, 2> { [3, 4].into_iter() }
}
fn main() {
    let result: Vec<u8> = Source::values().collect();
    assert_eq!(result, vec![3, 4]);
    println!("concrete return refines trait opaque return");
}
