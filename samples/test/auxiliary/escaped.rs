extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro]
pub fn escaped(_: TokenStream) -> TokenStream {
    r#"("a\n\t\"\\\x41\u{1f600}", b"\x00\xff\t")"#.parse().unwrap()
}

#[proc_macro]
pub fn unicode_underscore(_: TokenStream) -> TokenStream {
    r#""\u{1_f600}""#.parse().unwrap()
}
