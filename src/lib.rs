//! A small macro library providing safe, easy, and zero-cost way to construct
//! constant or literal instances of the `NonZero*` types from [`core::num`].
//!
//! The parameters to the macro must be constants expressions, but they can be
//! arbitrary arithmetic, calls to const fns, etc. They're fully evaluated and
//! checked for `NonZero`-ness at compile time, and thus have truly no cost —
//! even in debug mode.
//!
//! All misuse is detected at compile time, which can make it much easier to
//! audit and be confident there's no problems with the complex expression used
//! to initialized one `NonZero` or another.
//!
//! # Overview
//!
//! This crate provides 12 macros for constructing constants, one for each
//! non-zero integral type.
//!
//! - [`nonzero_lit::usize!`](crate::usize), producing a
//!   [`core::num::NonZeroUsize`].
//! - [`nonzero_lit::isize!`](crate::isize), producing a
//!   [`core::num::NonZeroIsize`].
//! - [`nonzero_lit::u128!`](crate::u128), producing a
//!   [`core::num::NonZeroU128`].
//! - [`nonzero_lit::i128!`](crate::i128), producing a
//!   [`core::num::NonZeroI128`].
//! - [`nonzero_lit::u64!`](crate::u64), producing a [`core::num::NonZeroU64`].
//! - [`nonzero_lit::i64!`](crate::i64), producing a [`core::num::NonZeroI64`].
//! - [`nonzero_lit::u32!`](crate::u32), producing a [`core::num::NonZeroU32`].
//! - [`nonzero_lit::i32!`](crate::i32), producing a [`core::num::NonZeroI32`].
//! - [`nonzero_lit::u16!`](crate::u16), producing a [`core::num::NonZeroU16`].
//! - [`nonzero_lit::i16!`](crate::i16), producing a [`core::num::NonZeroI16`].
//! - [`nonzero_lit::u8!`](crate::u8), producing a [`core::num::NonZeroU8`].
//! - [`nonzero_lit::i8!`](crate::i8), producing a [`core::num::NonZeroI8`].
//!
//! # Features
//!
//! - Crate fully supports `no_std`.
//! - All `NonZero` types are supported.
//! - Fully zero cost, even for debug builds (we always evaluate the constant as
//!   a `const`).
//! - Input to the macros can be arbitrary constant expressions. This includes
//!   `const fn` calls, which would be more difficult to verify the result as
//!   non-zero by hand.
//! - Misuse (trying to make a `NonZero$Int` with a zero value) is always
//!   detected at compile time, even when the macro is not being used to
//!   initialize a constant.
//! - No unsafe code.
//!
//! # Examples
//!
//! ### Basic usage
//!
//! ```
//! let x = nonzero_lit::i32!(4);
//! assert_eq!(x.get(), 4);
//! ```
//!
//! ### Constants
//!
//! #### Basic Constants
//! ```
//! const FERRIS: core::num::NonZeroU32 = nonzero_lit::u32!(0xf34415);
//! assert_eq!(FERRIS.get(), 0xf34415);
//! ```
//!
//! #### Use in `const fn`
//!
//! There's no restriction on use in `const fn` (as there are with some constant
//! macros):
//! ```
//! const fn get_magic() -> core::num::NonZeroU32 {
//!     nonzero_lit::u32!(25252)
//! }
//! assert_eq!(get_magic().get(), 25252)
//! ```
//!
//! However, note that the parameters to the macro still must be constants.
//!
//! #### Complex Expressions
//!
//! Arbitrary constant expressions (including `const fn`s) are allowed.
//!
//! ```
//! use core::num::NonZeroU64;
//!
//! const MASK: NonZeroU64 = nonzero_lit::u64!(0xffff_00ff_00ff_0f0f);
//! const MASKEE: NonZeroU64 = nonzero_lit::u64!(0xaabb_ccdd_eeff_1122);
//! // Note: as the complexity increases, it might become less trivial to
//! // verify that the result of an expression is not zero.
//! const MASKED: NonZeroU64 = nonzero_lit::u64!(MASK.get() & MASKEE.get());
//! assert_eq!(MASKED.get(), 0xaabb_00dd_00ff_0102_u64);
//! ```
//!
//! ### Zero Detection
//!
//! These tests are deliberately fail to compile, to demonstrate that we detect
//! misuse at compile time.
//!
//! #### Basic Zero Detection
//!
//! If an attempt to construct a zero NonZero is made, a compile error is
//! emitted.
//! ```compile_fail
//! const OH_NO: core::num::NonZeroU8 = nonzero_lit::i8!(0);
//! # let _ = OH_NO; // silence unused warning
//! ```
//!
//! #### Complex and Non-`const` Zero Detection
//! Zero detection works for complex expressions, even when not initializing a
//! constant (and even both simultaneously).
//! ```compile_fail
//! let shucks = nonzero_lit::i128!(i128::MIN.count_zeros());
//! # let _ = shucks; // silence unused warning
//! ```
//!
//! #### Robust against disabling `const_err` lint
//! Zero detection even works in the face of `#[allow(const_err)]` (which can
//! frequently be used to bypast const evaluation checks of this sort).
//! ```compile_fail
//! #![allow(const_err)]
//! use core::num::NonZeroU16;
//!
//! const UH_OH: NonZeroU16 = nonzero_lit::u16!(30 / !0);
//! # let _ = UH_OH; // silence unused warning
//! ```
#![no_std]
#![forbid(unsafe_code)]

/// Create a literal [`NonZeroUsize`](core::num::NonZeroUsize).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::usize!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: usize = 5;
/// const B: core::num::NonZeroUsize = nonzero_lit::usize!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroUsize = nonzero_lit::usize!(0);
/// ```
///
/// Macro is robust in the face of `#[allow(...)]`.
/// ```compile_fail
/// # use nonzero_lit::usize;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroUsize = nonzero_lit::usize!(0);
/// ```
///
///
/// ```compile_fail
/// let zero = nonzero_lit::usize!(usize::MAX.wrapping_add(1));
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::usize;
/// let bar = 3;
/// let foo = nonzero_lit::usize!(bar);
/// ```
#[macro_export]
macro_rules! usize {
    ($val:expr $(,)?) => {{
        const __E: usize = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroUsize = $crate::_private::nz_usize(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroIsize`](core::num::NonZeroIsize).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::isize!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: isize = 5;
/// const B: core::num::NonZeroIsize = nonzero_lit::isize!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroIsize = nonzero_lit::isize!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::isize;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroIsize = nonzero_lit::isize!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::isize;
/// let bar = 3;
/// let foo = nonzero_lit::isize!(bar);
/// ```
#[macro_export]
macro_rules! isize {
    ($val:expr $(,)?) => {{
        const __E: isize = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroIsize = $crate::_private::nz_isize(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroU8`](core::num::NonZeroU8).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::u8!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: u8 = 5;
/// const B: core::num::NonZeroU8 = nonzero_lit::u8!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroU8 = nonzero_lit::u8!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::u8;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroU8 = nonzero_lit::u8!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::u8;
/// let bar = 3;
/// let foo = nonzero_lit::u8!(bar);
/// ```
#[macro_export]
macro_rules! u8 {
    ($val:expr $(,)?) => {{
        const __E: u8 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroU8 = $crate::_private::nz_u8(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroI8`](core::num::NonZeroI8).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::i8!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: i8 = 5;
/// const B: core::num::NonZeroI8 = nonzero_lit::i8!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroI8 = nonzero_lit::i8!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::i8;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroI8 = nonzero_lit::i8!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::i8;
/// let bar = 3;
/// let foo = nonzero_lit::i8!(bar);
/// ```
#[macro_export]
macro_rules! i8 {
    ($val:expr $(,)?) => {{
        const __E: i8 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroI8 = $crate::_private::nz_i8(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroU16`](core::num::NonZeroU16).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::u16!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: u16 = 5;
/// const B: core::num::NonZeroU16 = nonzero_lit::u16!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroU16 = nonzero_lit::u16!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::u16;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroU16 = nonzero_lit::u16!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::u16;
/// let bar = 3;
/// let foo = nonzero_lit::u16!(bar);
/// ```
#[macro_export]
macro_rules! u16 {
    ($val:expr $(,)?) => {{
        const __E: u16 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroU16 = $crate::_private::nz_u16(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroI16`](core::num::NonZeroI16).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::i16!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: i16 = 5;
/// const B: core::num::NonZeroI16 = nonzero_lit::i16!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroI16 = nonzero_lit::i16!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::i16;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroI16 = nonzero_lit::i16!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::i16;
/// let bar = 3;
/// let foo = nonzero_lit::i16!(bar);
/// ```
#[macro_export]
macro_rules! i16 {
    ($val:expr $(,)?) => {{
        const __E: i16 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroI16 = $crate::_private::nz_i16(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroU32`](core::num::NonZeroU32).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::u32!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: u32 = 5;
/// const B: core::num::NonZeroU32 = nonzero_lit::u32!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroU32 = nonzero_lit::u32!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::u32;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroU32 = nonzero_lit::u32!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::u32;
/// let bar = 3;
/// let foo = nonzero_lit::u32!(bar);
/// ```
#[macro_export]
macro_rules! u32 {
    ($val:expr $(,)?) => {{
        const __E: u32 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroU32 = $crate::_private::nz_u32(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroI32`](core::num::NonZeroI32).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::i32!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: i32 = 5;
/// const B: core::num::NonZeroI32 = nonzero_lit::i32!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroI32 = nonzero_lit::i32!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::i32;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroI32 = nonzero_lit::i32!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::i32;
/// let bar = 3;
/// let foo = nonzero_lit::i32!(bar);
/// ```
#[macro_export]
macro_rules! i32 {
    ($val:expr $(,)?) => {{
        const __E: i32 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroI32 = $crate::_private::nz_i32(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroU64`](core::num::NonZeroU64).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::u64!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: u64 = 5;
/// const B: core::num::NonZeroU64 = nonzero_lit::u64!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroU64 = nonzero_lit::u64!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::u64;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroU64 = nonzero_lit::u64!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::u64;
/// let bar = 3;
/// let foo = nonzero_lit::u64!(bar);
/// ```
#[macro_export]
macro_rules! u64 {
    ($val:expr $(,)?) => {{
        const __E: u64 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroU64 = $crate::_private::nz_u64(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroI64`](core::num::NonZeroI64).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::i64!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: i64 = 5;
/// const B: core::num::NonZeroI64 = nonzero_lit::i64!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroI64 = nonzero_lit::i64!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::i64;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroI64 = nonzero_lit::i64!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::i64;
/// let bar = 3;
/// let foo = nonzero_lit::i64!(bar);
/// ```
#[macro_export]
macro_rules! i64 {
    ($val:expr $(,)?) => {{
        const __E: i64 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroI64 = $crate::_private::nz_i64(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroU128`](core::num::NonZeroU128).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::u128!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: u128 = 5;
/// const B: core::num::NonZeroU128 = nonzero_lit::u128!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroU128 = nonzero_lit::u128!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::u128;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroU128 = nonzero_lit::u128!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::u128;
/// let bar = 3;
/// let foo = nonzero_lit::u128!(bar);
/// ```
#[macro_export]
macro_rules! u128 {
    ($val:expr $(,)?) => {{
        const __E: u128 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroU128 = $crate::_private::nz_u128(__E);
            NZ
        }
    }};
}

/// Create a literal [`NonZeroI128`](core::num::NonZeroI128).
///
/// # Examples
/// Basic usage
/// ```
/// let x = nonzero_lit::i128!(4);
/// assert_eq!(x.get(), 4);
/// ```
///
/// Works for consts, and the parameter can be any const expression (not just a
/// literal).
/// ```
/// const A: i128 = 5;
/// const B: core::num::NonZeroI128 = nonzero_lit::i128!(A * 10);
/// assert_eq!(B.get(), 50);
/// ```
///
/// Misuse is detected at compile time.
/// ```compile_fail
/// const ZERO: core::num::NonZeroI128 = nonzero_lit::i128!(0);
/// ```
///
/// Even if dodgy code tries to `#[allow(...)]` it.
/// ```compile_fail
/// # use nonzero_lit::i128;
/// #[allow(const_err)]
/// const ZERO: core::num::NonZeroI128 = nonzero_lit::i128!(0);
/// ```
///
/// Note: argument must be a constant expression.
/// ```compile_fail
/// # use nonzero_lit::i128;
/// let bar = 3;
/// let foo = nonzero_lit::i128!(bar);
/// ```
#[macro_export]
macro_rules! i128 {
    ($val:expr $(,)?) => {{
        const __E: i128 = $val;
        {
            #[deny(const_err)]
            const NZ: $crate::_private::NonZeroI128 = $crate::_private::nz_i128(__E);
            NZ
        }
    }};
}

// Implementation detail — not part of public API.
#[doc(hidden)]
pub mod _private {
    pub use core::num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    };

    macro_rules! define_nz_ctor {
        ($(pub fn $nz_func:ident($n:ident : $int:ident) -> $NonZeroInt:ident;)+) => {$(
            #[inline]
            pub const fn $nz_func($n : $int) -> $NonZeroInt {
                // Note: Hacky const fn assert.
                let _ = ["N must not be zero"][($n == 0) as usize];

                match $NonZeroInt::new($n) {
                    Some(x) => x,
                    // The assert above makes this branch unreachable
                    None => loop {},
                }
            }
        )+};
    }

    define_nz_ctor! {
        pub fn nz_usize(n: usize) -> NonZeroUsize;
        pub fn nz_isize(n: isize) -> NonZeroIsize;
        pub fn nz_u8(n: u8) -> NonZeroU8;
        pub fn nz_i8(n: i8) -> NonZeroI8;
        pub fn nz_u16(n: u16) -> NonZeroU16;
        pub fn nz_i16(n: i16) -> NonZeroI16;
        pub fn nz_u32(n: u32) -> NonZeroU32;
        pub fn nz_i32(n: i32) -> NonZeroI32;
        pub fn nz_u64(n: u64) -> NonZeroU64;
        pub fn nz_i64(n: i64) -> NonZeroI64;
        pub fn nz_u128(n: u128) -> NonZeroU128;
        pub fn nz_i128(n: i128) -> NonZeroI128;
    }
}
