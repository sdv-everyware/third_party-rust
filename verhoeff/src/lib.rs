//! An implementation of the Verhoeff algorithm.
//!
//! This checksum algorithm is not particularly common (the simpler and somewhat inferior Luhn
//! algorithm is much more widely used, e.g. in credit card numbers), but it definitely gets some
//! use; for example, India’s Aadhaar biometric identity system uses 12-digit numbers as the ID
//! number, with the final digit being a Verhoeff checksum.
//!
//! Background reading: <https://en.wikipedia.org/wiki/Verhoeff_algorithm>

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// The multiplication table.
static D: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];

/// The permutation table.
static P: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
    [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
    [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
    [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];

/// The inverse table.
static INV: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

/// Methods to perform the Verhoeff algorithm.
///
/// This is implemented for strings (which must contain only the ASCII digits 0–9) and u8 slices
/// (which must contain only the numbers 0–9).
///
/// ## Examples
///
/// ```
/// use verhoeff::Verhoeff;
/// assert_eq!("12345".calculate_verhoeff_check_digit(), 1);
/// assert!("123451".validate_verhoeff_check_digit());
/// ```
pub trait Verhoeff {
    /// Confirm that the digits are valid and that the last is the correct check digit.
    ///
    /// If any digit is not valid (a character other than ASCII digits 0–9 in strings, or a
    /// number greater than nine in `u8` slices), it returns false.
    fn validate_verhoeff_check_digit(&self) -> bool;

    /// Calculate the check digit for this input.
    ///
    /// If any digit is not valid (a character other than ASCII digits 0–9 in strings, or a
    /// number greater than nine in `u8` slices), it will panic.
    ///
    /// The return value is an integer in the range 0–9.
    fn calculate_verhoeff_check_digit(&self) -> u8;
}

impl Verhoeff for str {
    fn validate_verhoeff_check_digit(&self) -> bool {
        let mut c = 0;
        for (i, digit) in self.bytes().rev().enumerate() {
            let digit = match digit {
                b'0'..=b'9' => digit - b'0',
                _ => return false,
            };
            c = D[c][P[i % 8][digit as usize] as usize] as usize;
        }

        c == 0
    }

    fn calculate_verhoeff_check_digit(&self) -> u8 {
        let mut c = 0;
        for (i, digit) in self.bytes().rev().enumerate() {
            assert!(
                matches!(digit, b'0'..=b'9'),
                "number string contains a non-digit character",
            );
            c = D[c][P[(i + 1) % 8][(digit - b'0') as usize] as usize] as usize;
        }

        INV[c]
    }
}

impl Verhoeff for [u8] {
    fn validate_verhoeff_check_digit(&self) -> bool {
        let mut c = 0;
        for (i, &digit) in self.iter().rev().enumerate() {
            if digit > 9 {
                return false;
            }
            c = D[c][P[i % 8][digit as usize] as usize] as usize;
        }

        c == 0
    }

    fn calculate_verhoeff_check_digit(&self) -> u8 {
        let mut c = 0;
        for (i, &digit) in self.iter().rev().enumerate() {
            assert!(digit < 10, "number sequence contains a value above nine");
            c = D[c][P[(i + 1) % 8][digit as usize] as usize] as usize;
        }

        INV[c]
    }
}

/// A convenience trait for mutable methods pertaining to the Verhoeff algorithm.
///
#[cfg_attr(
    feature = "alloc",
    doc = " This is implemented for `String` and `Vec<u8>`.",
)]
#[cfg_attr(
    not(feature = "alloc"),
    doc = " This is normally implemented for `String` and `Vec<u8>`, but you’ve you’ve disabled \
        the alloc feature, so it’s not implemented on anything out of the box.",
)]
pub trait VerhoeffMut {
    /// A convenience method to apply the Verhoeff algorithm in-place.
    ///
    /// ## Example
    ///
    #[cfg_attr(feature = "alloc", doc = " ```")]
    #[cfg_attr(not(feature = "alloc"), doc = " ```rust,ignore")]
    /// use verhoeff::VerhoeffMut;
    /// let mut digits = String::from("12345");
    /// digits.push_verhoeff_check_digit();
    /// assert_eq!(digits, "123451");
    #[doc = " ```"]  // just to keep the Vim syntax highlighter happy. Silly, huh?
    fn push_verhoeff_check_digit(&mut self);
}

#[cfg(feature = "alloc")]
impl VerhoeffMut for alloc::string::String {
    fn push_verhoeff_check_digit(&mut self) {
        self.push(char::from(b'0' + self.calculate_verhoeff_check_digit()));
    }
}

#[cfg(feature = "alloc")]
impl VerhoeffMut for alloc::vec::Vec<u8> {
    /// A convenience method to apply the Verhoeff algorithm in-place.
    ///
    /// ## Example
    ///
    /// ```
    /// use verhoeff::VerhoeffMut;
    /// let mut digits = vec![1, 2, 3, 4, 5];
    /// digits.push_verhoeff_check_digit();
    /// assert_eq!(digits, [1, 2, 3, 4, 5, 1]);
    /// ```
    fn push_verhoeff_check_digit(&mut self) {
        self.push(self.calculate_verhoeff_check_digit());
    }
}

/// A convenience method to calculate the Verhoeff algorithm check digit.
///
/// You can pass in a `&str` or a `&[u8]`, and get back a number in the range 0–9.
///
/// This is equivalent to calling [`Verhoeff::calculate_verhoeff_check_digit`].
/// It will panic if the input is not comprised of appropriate digits.
#[inline]
pub fn calculate<T: Verhoeff + ?Sized>(input: &T) -> u8 {
    input.calculate_verhoeff_check_digit()
}

/// A convenience method to validate against the Verhoeff algorithm.
///
/// The check digit is presumed to be at the end of the input.
///
/// You can pass in a `&str` or a `&[u8]`.
///
/// This is equivalent to calling [`Verhoeff::validate_verhoeff_check_digit`].
/// It will return false if the input is not comprised of appropriate digits.
#[inline]
pub fn validate<T: Verhoeff + ?Sized>(input: &T) -> bool {
    input.validate_verhoeff_check_digit()
}

#[test]
fn test() {
    for full in [
        "0",
        "2363",
        "758722",
        "123451",
        "1428570",
        "1234567890120",
        "84736430954837284567892",
    ] {
        let sans_check_digit = &full[..full.len() - 1];
        let expected_check_digit = full.chars().last().unwrap() as u32 as u8 - b'0';
        let actual_check_digit = calculate(sans_check_digit);
        if expected_check_digit != actual_check_digit {
            panic!(
                "On {}, expected check digit {} but calculated {}",
                sans_check_digit, expected_check_digit, actual_check_digit,
            );
        }
        if !validate(full) {
            panic!("{} failed to validate", full);
        }
        #[cfg(feature = "alloc")]
        {
            let mut new = alloc::string::String::from(sans_check_digit);
            new.push_verhoeff_check_digit();
            assert_eq!(new, full);
        }
    }
    for full in [
        &[0],
        &[2, 3, 6, 3],
        &[7, 5, 8, 7, 2, 2],
        &[1, 2, 3, 4, 5, 1],
        &[1, 4, 2, 8, 5, 7, 0],
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 0],
        &[
            8, 4, 7, 3, 6, 4, 3, 0, 9, 5, 4, 8, 3, 7, 2, 8, 4, 5, 6, 7, 8, 9, 2,
        ],
    ] as [&[u8]; 7] {
        let sans_check_digit = &full[..full.len() - 1];
        let expected_check_digit = *full.iter().last().unwrap();
        let actual_check_digit = calculate(sans_check_digit);
        if expected_check_digit != actual_check_digit {
            panic!(
                "On {:?}, expected check digit {:?} but calculated {:?}",
                sans_check_digit, expected_check_digit, actual_check_digit,
            );
        }
        if !validate(full) {
            panic!("{:?} failed to validate", full);
        }
        #[cfg(feature = "alloc")]
        {
            let mut new = alloc::vec::Vec::from(sans_check_digit);
            new.push_verhoeff_check_digit();
            assert_eq!(new, full);
        }
    }
    assert!(!validate("122451"));
    assert!(!"128451".validate_verhoeff_check_digit());
    assert!(![1, 2, 4, 3, 5, 1].validate_verhoeff_check_digit());
}

#[test]
fn malformed_inputs_1() {
    assert!([].validate_verhoeff_check_digit());
    assert!("".validate_verhoeff_check_digit());
    assert!(![1, 2, 3, 4, 10, 5, 6].validate_verhoeff_check_digit());
    assert!(!validate("1234∞56"));
}

#[test]
#[should_panic]
fn malformed_inputs_2() {
    [1, 2, 3, 4, 10, 5, 6].calculate_verhoeff_check_digit();
}

#[test]
#[should_panic]
fn malformed_inputs_3() {
    calculate("1234∞56");
}
