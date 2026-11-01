// compile-flags: --edition 2021
const fn length(text: &str) -> usize { text.len() }
const unsafe fn wrap(data: &[u32]) -> &[u32] { data }
macro_rules! words {
    ($length:expr) => {{
        const LENGTH: usize = $length;
        const DATA: [u32; LENGTH] = {
            let mut values: [u32; LENGTH] = [0; LENGTH];
            let mut i = 0;
            while i < LENGTH { values[i] = i as u32; i += 1; }
            values
        };
        unsafe { wrap(&DATA) }
    }};
}
struct Entry { words: &'static [u32] }
const ENTRIES: &[Entry] = &[Entry { words: words!(length("twelve bytes")) }];
struct State { values: [bool; ENTRIES.len()] }
fn main() {
    assert_eq!(ENTRIES[0].words, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(words!(0).len(), 0);
    assert_eq!(words!(1), &[0]);
    assert_eq!(words!(257)[256], 256);
    println!("constant array local types pass");
}
