extern crate proc_macro;
extern crate helper_export;
use helper_export::derive as helper_derive;
#[proc_macro_attribute]
pub fn derive(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assert_eq!(helper_derive!(), 77);
    item
}
