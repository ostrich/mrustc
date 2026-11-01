// compile-flags: --edition 2021
macro_rules! check_width {
    ($module:ident, $repr:ident, $low:expr, $high:expr) => {
        mod $module {
            #[repr($repr)]
            enum Tag { Low = $low, Minus = -1, Zero = 0, High = $high }
            const fn read(tag: Tag) -> u8 {
                match tag { Tag::Low => 1, Tag::Minus => 2, Tag::Zero => 3, Tag::High => 4 }
            }
            const VALUES: [u8; 4] = [read(Tag::Low), read(Tag::Minus), read(Tag::Zero), read(Tag::High)];
            pub fn check() { assert_eq!(VALUES, [1, 2, 3, 4]); }
        }
    };
}
check_width!(byte, i8, -128, 127);
check_width!(half, i16, -32768, 32767);
check_width!(word, i32, -2147483648, 2147483647);
check_width!(double, i64, -9223372036854775808, 9223372036854775807);
fn main() { byte::check(); half::check(); word::check(); double::check(); }
