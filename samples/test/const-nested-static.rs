// compile-flags: --edition 2021
struct Entry { name: &'static [char] }
const TABLE: &[Entry] = &[
    Entry { name: &['a', 'b'] },
    Entry { name: &['c', 'd'] },
];
const fn validate() -> bool {
    let left = TABLE[0].name;
    let right = TABLE[1].name;
    left[0] == 'a' && left[1] == 'b' && right[0] == 'c' && right[1] == 'd'
}
const VALID: bool = validate();
fn main() {
    assert!(VALID);
    println!("nested static relocation reads pass");
}
