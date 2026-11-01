pub trait Generic<T> { fn values(value: T) -> impl Iterator<Item = T>; }
pub trait Method { fn values<T>(value: T) -> impl Iterator<Item = T>; }
pub trait Nested { fn values(value: u8) -> (impl Iterator<Item = u8>, u8); }
