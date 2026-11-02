#[cfg(mrustc)]
mod marker {
    #[lang = "sized"]
    pub trait Sized: MetaSized {}
    #[lang = "meta_sized"]
    pub trait MetaSized: PointeeSized {}
    #[lang = "pointee_sized"]
    pub trait PointeeSized {}
    #[lang = "copy"]
    pub trait Copy {}
    impl Copy for u8 {}
    impl Copy for u16 {}
    impl Copy for u32 {}
    impl Copy for u64 {}
    impl Copy for u128 {}
    impl Copy for usize {}
    impl Copy for f32 {}
    impl Copy for f64 {}
    impl Copy for char {}
    impl<T: ?Sized> Copy for *const T {}
    impl<T: ?Sized> Copy for &T {}
}
#[cfg(mrustc)]
pub mod ops {
    pub struct RangeFull;
}
#[cfg(mrustc)]
extern "rust-intrinsic" {
    fn size_of<T>() -> usize;
    fn min_align_of<T>() -> usize;
    fn transmute<T, U>(value: T) -> U;
}
#[cfg(not(mrustc))]
use core::mem::{align_of as min_align_of, size_of, transmute};
#[cfg(mrustc)]
#[lang = "unsafe_cell"]
#[repr(transparent)]
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
#[cfg(mrustc)]
#[lang = "add"]
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
#[cfg(mrustc)]
impl Add for f64 {
    type Output = f64;
    fn add(self, rhs: f64) -> f64 {
        self + rhs
    }
}
#[cfg(mrustc)]
#[lang = "mul"]
pub trait Mul<Rhs = Self> {
    type Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
}
#[cfg(mrustc)]
impl Mul for f32 {
    type Output = f32;
    fn mul(self, rhs: f32) -> f32 {
        self * rhs
    }
}
#[cfg(mrustc)]
use marker::Sized;
#[cfg(mrustc)]
#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}
#[cfg(mrustc)]
#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T: ?Sized> {}
#[cfg(mrustc)]
#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}
#[cfg(mrustc)]
#[lang = "neg"]
pub trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}
#[cfg(mrustc)]
impl Neg for f32 {
    type Output = f32;
    fn neg(self) -> f32 {
        -self
    }
}
#[cfg(mrustc)]
impl Neg for f64 {
    type Output = f64;
    fn neg(self) -> f64 {
        -self
    }
}
#[cfg(mrustc)]
#[lang = "fn_ptr_trait"]
pub trait FnPtr {
    #[lang = "fn_ptr_addr"]
    fn addr(self) -> *const ();
}
#[cfg(mrustc)]
#[lang = "discriminant_kind"]
pub trait DiscriminantKind {
    type Discriminant;
}
#[cfg(mrustc)]
#[lang = "pointee_trait"]
pub trait Pointee {
    #[lang = "metadata_type"]
    type Metadata;
}
#[cfg(mrustc)]
#[lang = "tuple_trait"]
pub trait Tuple {}
#[cfg(mrustc)]
#[lang = "shr"]
pub trait Shr<Rhs = Self> {
    type Output;
    fn shr(self, rhs: Rhs) -> Self::Output;
}
#[cfg(mrustc)]
impl Shr for u128 {
    type Output = u128;
    fn shr(self, rhs: u128) -> u128 {
        self >> rhs
    }
}
