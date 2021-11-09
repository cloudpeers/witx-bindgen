pub use crate::futures::*;

macro_rules! as_traits {
    ($(($trait_:ident $func:ident $ty:ident <=> $($tys:ident)*))*) => ($(
        pub fn $func<T: $trait_>(t: T) -> $ty {
            t.$func()
        }

        pub trait $trait_ {
            fn $func(self) -> $ty;
        }

        impl<'a, T: Copy + $trait_> $trait_ for &'a T {
            fn $func(self) -> $ty{
                (*self).$func()
            }
        }

        $(
            impl $trait_ for $tys {
                #[inline]
                fn $func(self) -> $ty {
                    self as $ty
                }
            }
        )*

    )*)
}

as_traits! {
    (AsI64 as_i64 i64 <=> i64 u64)
    (AsI32 as_i32 i32 <=> i32 u32 i16 u16 i8 u8 char usize)
    (AsF32 as_f32 f32 <=> f32)
    (AsF64 as_f64 f64 <=> f64)
}
