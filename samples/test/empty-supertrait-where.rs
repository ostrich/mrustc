// compile-flags: --edition 2021
trait RefValue { fn value(self) -> usize; }
impl<'a> RefValue for &'a str { fn value(self) -> usize { self.len() } }
trait Measure: where for<'a> &'a Self: RefValue {
    fn measured(&self) -> usize { self.value() }
}
impl Measure for str {}
trait Empty: {}
impl Empty for str {}
trait WithTrailing: Empty + where for<'a> &'a Self: RefValue {}
impl WithTrailing for str {}
fn main() {
    assert_eq!("abc".measured(), 3);
    println!("empty supertrait lists pass");
}
