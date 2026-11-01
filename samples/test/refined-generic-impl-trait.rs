// aux-build: generic_trait.rs --crate-type rlib --edition 2021
// compile-flags: --edition 2021
extern crate generic_trait;
struct Source;
impl<T> generic_trait::Generic<T> for Source {
    fn values(value: T) -> std::iter::Once<T> { std::iter::once(value) }
}
impl generic_trait::Method for Source {
    fn values<T>(value: T) -> std::iter::Once<T> { std::iter::once(value) }
}
impl generic_trait::Nested for Source {
    fn values(value: u8) -> (std::iter::Once<u8>, u8) { (std::iter::once(value), 9) }
}
fn main() {
    assert_eq!(<Source as generic_trait::Generic<_>>::values(7).collect::<Vec<_>>(), vec![7]);
    assert_eq!(<Source as generic_trait::Method>::values(11).collect::<Vec<_>>(), vec![11]);
    let (iter, n) = <Source as generic_trait::Nested>::values(3);
    assert_eq!(iter.collect::<Vec<_>>(), vec![3]);
    assert_eq!(n, 9);
}
