// compile-flags: --edition 2021
macro_rules! classify { ($x:literal) => { 1 }; ($x:expr) => { 2 }; }
macro_rules! forward { ($x:expr) => { classify!($x) }; }
macro_rules! again { ($x:expr) => { forward!($x) }; }
macro_rules! negative { ($x:expr) => { forward!(-$x) }; }
macro_rules! attributed { ($x:expr) => { forward!(#[allow(unused)] $x) }; }
macro_rules! twice { ($x:expr) => { (classify!($x), classify!($x)) }; }
fn main() {
    assert_eq!(twice!((1)), (2, 2));
    assert_eq!(twice!(1), (1, 1));
    assert_eq!(classify!(c"cstring"), 1);
    assert_eq!(classify!(-true), 1);
    assert_eq!(classify!(-"text"), 1);
    assert_eq!(forward!(c"cstring"), 1);
    assert_eq!(forward!(-c"cstring"), 1);
    assert_eq!(forward!('x'), 1);
    assert_eq!(forward!(b'x'), 1);
    let bare = forward!(1);
    let negative_bare = forward!(-1);
    let grouped = forward!((1));
    let double_grouped = forward!(((1)));
    let negative_group = forward!(-(1));
    let grouped_negative = forward!((-1));
    let negative_negative = forward!(-(-1));
    let forwarded_group = again!((1));
    let forwarded_bare = again!(1);
    let generated_negative = negative!(1);
    let generated_group = negative!((1));
    let attribute = attributed!(1);
    let float = forward!(-1.5);
    let grouped_float = forward!(-(1.5));
    let bytes = forward!(b"bytes");
    let boolean = forward!(true);
    let string = forward!("text");
    let negative_bool = forward!(-true);
    let negative_string = forward!(-"text");
    let negative_bytes = forward!(-b"bytes");
    let negative_char = forward!(-'x');
    let generated_bool = negative!(false);
    assert_eq!([negative_bool, negative_string, negative_bytes, negative_char, generated_bool], [1; 5]);
    let arithmetic = forward!(1 + 2);
    assert_eq!([bare, negative_bare, forwarded_bare, generated_negative, attribute, float, bytes, boolean, string], [1; 9]);
    assert_eq!([grouped, double_grouped, negative_group, grouped_negative, negative_negative, forwarded_group, generated_group, grouped_float, arithmetic], [2; 9]);
}
