use std::borrow::Cow;
fn names() -> &'static [Cow<'static, str>] {
    &[Cow::Borrowed("unknown"), Cow::Borrowed("linux")]
}
fn other() -> &'static [Cow<'static, str>] {
    &[]
}
struct Root {
    bytes: [u8; 3],
    value: Cow<'static, str>,
}
const ROOT: Root = Root {
    bytes: [1, 2, 3],
    value: Cow::Borrowed("borrowed"),
};
fn bytes() -> &'static [u8; 3] {
    &ROOT.bytes
}
fn bytes_other() -> &'static [u8; 3] {
    &[0, 0, 0]
}
fn main() {
    let callbacks: [fn() -> &'static [Cow<'static, str>]; 2] = [names, other];
    let values = callbacks[std::env::args().count() - 1]();
    let byte_callbacks: [fn() -> &'static [u8; 3]; 2] = [bytes, bytes_other];
    let byte_values = byte_callbacks[std::env::args().count() - 1]();
    let marker = format!("{} {}", 123456, 987654);
    assert_eq!(marker, "123456 987654");
    assert_eq!(byte_values, &[1, 2, 3]);
    assert_eq!(values.len(), 2);
    assert_eq!(values[0].as_ref(), "unknown");
    assert_eq!(values[1].as_ref(), "linux");
}
