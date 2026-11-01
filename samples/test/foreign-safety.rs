// compile-flags: --edition 2024
unsafe extern "C" {
    #[link_name = "abs"] safe fn safe_abs(value: i32) -> i32;
    #[link_name = "abs"] unsafe fn unsafe_abs(value: i32) -> i32;
}
fn main() {
    let safe_pointer: extern "C" fn(i32) -> i32 = safe_abs;
    let unsafe_pointer: unsafe extern "C" fn(i32) -> i32 = unsafe_abs;
    assert_eq!(safe_pointer(-4), 4);
    assert_eq!(unsafe { unsafe_pointer(-7) }, 7);
    println!("foreign safety qualifiers pass");
}
