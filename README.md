# `nonzero_lit`
[![Docs](https://docs.rs/nonzero_lit/badge.svg)](https://docs.rs/nonzero_lit)
[![Latest Version](https://img.shields.io/crates/v/nonzero_lit.svg)](https://crates.io/crates/nonzero_lit)
![Minimum Rust Version](https://img.shields.io/badge/MSRV%201.51-blue.svg)

A small macro library providing safe, easy, and fully zero-cost way to construct constant or literal instances of the `NonZero*` types from `core::num`.

## Features

- Crate fully supports `no_std`.
- All `NonZero` types are supported:
    - [`core::num::NonZeroUsize`](https://doc.rust-lang.org/core/num/struct.NonZeroUsize.html) via the [`nonzero_lit::usize!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.usize.html) macro.
    - [`core::num::NonZeroIsize`](https://doc.rust-lang.org/core/num/struct.NonZeroIsize.html) via the [`nonzero_lit::isize!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.isize.html) macro.
    - [`core::num::NonZeroU128`](https://doc.rust-lang.org/core/num/struct.NonZeroU128.html) via the [`nonzero_lit::u128!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.u128.html) macro.
    - [`core::num::NonZeroI128`](https://doc.rust-lang.org/core/num/struct.NonZeroI128.html) via the [`nonzero_lit::i128!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.i128.html) macro.
    - [`core::num::NonZeroU64`](https://doc.rust-lang.org/core/num/struct.NonZeroU64.html) via the [`nonzero_lit::u64!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.u64.html) macro.
    - [`core::num::NonZeroI64`](https://doc.rust-lang.org/core/num/struct.NonZeroI64.html) via the [`nonzero_lit::i64!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.i64.html) macro.
    - [`core::num::NonZeroU32`](https://doc.rust-lang.org/core/num/struct.NonZeroU32.html) via the [`nonzero_lit::u32!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.u32.html) macro.
    - [`core::num::NonZeroI32`](https://doc.rust-lang.org/core/num/struct.NonZeroI32.html) via the [`nonzero_lit::i32!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.i32.html) macro.
    - [`core::num::NonZeroU16`](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU16.html) via the [`nonzero_lit::u16!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.u16.html) macro.
    - [`core::num::NonZeroI16`](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroI16.html) via the [`nonzero_lit::i16!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.i16.html) macro.
    - [`core::num::NonZeroU8`](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU8.html) via the [`nonzero_lit::u8!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.u8.html) macro.
    - [`core::num::NonZeroI8`](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroI8.html) via the [`nonzero_lit::i8!`](https://docs.rs/nonzero_lit/%2A/nonzero_lit/macro.i8.html) macro.

- Fully zero cost, even for debug builds — we always evaluate the constant as a `const`.
- Input to the macros can be arbitrary constant expressions. This includes `const fn` calls, which would be more difficult to verify the result as non-zero by hand.
- Misuse (trying to make a `NonZero$Int` with a zero value) is always detected at compile time, even when the macro is not being used to initialize a constant.
- Only one line of unsafe, trivially verifiable as correct (and hopefully avoidable in future versions of Rust).

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
nonzero_lit = "0.1"
```

### Examples

```rust
let x = nonzero_lit::i32!(4);
assert_eq!(x.get(), 4);
```

```rust
const FERRIS: core::num::NonZeroU32 = nonzero_lit::u32!(0xf34415);
assert_eq!(FERRIS.get(), 0xf34415);
```

```rust
const FERRIS: core::num::NonZeroU32 = nonzero_lit::u32!(0xf34415);
assert_eq!(FERRIS.get(), 0xf34415);
```

## License

Public domain, as explained [here](https://creativecommons.org/publicdomain/zero/1.0/legalcode). If that's unacceptable, it's also available under either the Apache-2.0 or MIT licenses, at your option.
