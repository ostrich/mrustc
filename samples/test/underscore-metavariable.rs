// compile-flags: --edition 2021
macro_rules! identity { ($_:tt) => { $_ }; }
macro_rules! with_dollar {
    ($($body:tt)*) => {
        macro_rules! expand { $($body)* }
        expand!($);
    };
}
with_dollar! {
    ($_:tt) => {
        macro_rules! second { ($_ value:expr) => { $_ value + 1 }; }
    };
}
fn main() {
    assert_eq!(identity!(42), 42);
    assert_eq!(second!(41), 42);
    println!("underscore metavariables pass");
}
