# The Verhoeff algorithm, for number checksums

An implementation of the Verhoeff algorithm.

This checksum algorithm is not particularly common (the simpler and somewhat inferior Luhn
algorithm is much more widely used, e.g. in credit card numbers), but it definitely gets some
use; for example, India’s Aadhaar biometric identity system uses 12-digit numbers as the ID
number, with the final digit being a Verhoeff checksum.

Background reading: <https://en.wikipedia.org/wiki/Verhoeff_algorithm>

----

## Examples

```rust
use verhoeff::Verhoeff;
assert_eq!("12345".calculate_verhoeff_check_digit(), 1);
assert!(verhoeff::validate(&[1, 2, 3, 4, 5, 1]));
assert!(!"123456".validate_verhoeff_check_digit());

use verhoeff::VerhoeffMut;
let mut digits = vec![1, 2, 3, 4, 5];
digits.push_verhoeff_check_digit();
assert_eq!(digits, [1, 2, 3, 4, 5, 1]);
```

## Cargo.toml usage and features

Standard:

```toml
[dependencies]
verhoeff = "1"
```

Disabling the `std` feature to get `#![no_std]`, but retaining `alloc` in order to not lose any functionality:

```toml
[dependencies]
verhoeff = { version = "1", default-features = false, features = ["alloc"] }
```

Disabling the `std` feature without reenabling `alloc`, thereby losing the implementations of `VerhoeffMut` (`push_verhoeff_check_digit`) for `String` and `Vec<u8>`:

```toml
[dependencies]
verhoeff = { version = "1", default-features = false }
```
