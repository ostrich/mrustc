use std::cell::Cell;
struct Container {
    bytes: [u8; 3],
    cell: Cell<u8>,
}
const ROOT: Container = Container {
    bytes: [7, 8, 9],
    cell: Cell::new(0),
};
fn main() {
    let a = &ROOT.bytes;
    assert_eq!(a[1], 8);
}
