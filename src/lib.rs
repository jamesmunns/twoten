#![cfg_attr(not(any(test, feature = "use-std")), no_std)]

//! # twoten
//!
//! Hash data into two bytes, and turn it into an (exactly)
//! ten byte, human readable summary.
//!
//! Summaries are:
//!
//! * A 16-bit fnv1a hash of the data
//! * The first 8 bits of the hash are used to select a
//!     6-character word. See [`WORDS`].
//! * The next 8 bits of the hash are formatted in an octal format, `000` to `377`.
//!
//! The resulting summary looks like `JESTER-123`
//!
//! ## Example
//!
//! ```rust
//! // `twoten_buf` is available on no-std and makes a stack string
//! use twoten::twoten_buf;
//!
//! let name = twoten_buf(b"Hello, world!");
//! assert_eq!("ALFRED-035", name.as_str());
//! // Supports AsRef conversions
//! let asref: &str = &name;
//! assert_eq!("ALFRED-035", asref);
//!
//! // `twoten_string` is available on std and makes a heap string.
//! // requires the `use-std` feature.
//! use twoten::twoten_string;
//!
//! let name: String = twoten_string(b"Hello, world!");
//! assert_eq!("ALFRED-035", name.as_str());
//! ```
//!
//! ## Features
//!
//! Enable the `use-std` feature to activate std helpers

use core::ops::Deref;

use words::{decode, WORDS};

pub mod words;

/// Hash the given data, and return bytes that represent the produced twoten string
///
/// ```rust
/// use twoten::twoten;
///
/// let name = twoten(b"Hello, world!");
/// assert_eq!(&[65u8, 76, 70, 82, 69, 68, 45, 48, 51, 53], name.as_slice());
/// assert_eq!("ALFRED-035", core::str::from_utf8(&name).unwrap());
/// ```
pub fn twoten(data: &[u8]) -> [u8; 10] {
    // Hash the data, and fold it into two bytes
    let hash32 = fnv1a_32(data);
    let hash16 = hash32 ^ (hash32 >> 16);
    let hash16 = (hash16 & 0xFFFF) as u16;
    let [name, oct] = hash16.to_le_bytes();

    twoten_from_parts(name, oct)
}

/// Hash the given data, and return a stack allocated buffer that contains
/// the string, and can be used without a heap
///
/// See [TwoTenString] for details about the container.
///
/// ```rust
/// use twoten::twoten_buf;
///
/// let name = twoten_buf(b"Hello, world!");
/// assert_eq!("ALFRED-035", name.as_str());
/// // Supports AsRef conversions
/// let asref: &str = &name;
/// assert_eq!("ALFRED-035", asref);
/// ```
pub fn twoten_buf(data: &[u8]) -> TwoTenString {
    let bytes = twoten(data);
    TwoTenString { buf: bytes }
}

/// Hashes data and produces a heap allocated string
///
/// ```rust
/// use twoten::twoten_string;
///
/// let name: String = twoten_string(b"Hello, world!");
/// assert_eq!("ALFRED-035", name.as_str());
/// ```
#[cfg(any(test, feature = "use-std"))]
pub fn twoten_string(data: &[u8]) -> String {
    let bytes = twoten(data);
    #[cfg(test)]
    let _ = core::str::from_utf8(&bytes).unwrap();

    // SAFETY: We exhaustively test every possible combo in tests
    let s = unsafe { core::str::from_utf8_unchecked(&bytes) };

    s.to_string()
}

/// A stack container for the generated name.
///
/// Implements common accessors, and a `to_str()` method.
///
/// Resulting `&str` is ALWAYS ten bytes in length, and only contains
/// ASCII data.
///
/// See [`twoten_buf()`] for creation
#[derive(PartialEq, Clone, PartialOrd, Hash)]
pub struct TwoTenString {
    buf: [u8; 10],
}

impl core::fmt::Debug for TwoTenString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl core::fmt::Display for TwoTenString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl TwoTenString {
    /// Get the str slice of the name. May contain leading spaces.
    ///
    /// Resulting `&str` is ALWAYS ten bytes in length, and only contains
    /// ASCII data.
    pub fn as_str(&self) -> &str {
        // SAFETY: We exhaustively test all possible combinations and
        // only allow creation from valid inputs
        unsafe {
            core::str::from_utf8_unchecked(&self.buf)
        }
    }
}

impl AsRef<str> for TwoTenString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for TwoTenString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg(any(test, feature = "use-std"))]
impl From<TwoTenString> for String {
    fn from(value: TwoTenString) -> Self {
        value.as_str().to_string()
    }
}

#[cfg(any(test, feature = "use-std"))]
impl From<&TwoTenString> for String {
    fn from(value: &TwoTenString) -> Self {
        value.as_str().to_string()
    }
}

fn twoten_from_parts(name: u8, octa: u8) -> [u8; 10] {
    // Lookup+decode the name.
    let name = WORDS[name as usize];
    let name = decode(name);
    // Decode the octal parts
    let octa = oct_bytes(octa);

    // build the resulting string
    let mut out = [0u8; 10];
    out[..6].copy_from_slice(&name);
    out[6] = b'-';
    out[7..].copy_from_slice(&octa);
    out
}

fn fnv1a_32(data: &[u8]) -> u32 {
    const PRIME: u32 = 0x01000193;
    const BASIS: u32 = 0x811c9dc5;
    let mut hash = BASIS;
    for b in data {
        hash ^= u32::from(*b);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn oct_bytes(val: u8) -> [u8; 3] {
    const LUT: [u8; 8] = *b"01234567";
    [
        LUT[(val >> 6) as usize],
        LUT[((val >> 3) & 0b111) as usize],
        LUT[(val & 0b111) as usize],
    ]
}

#[cfg(test)]
mod test {
    use crate::{twoten, twoten_from_parts};

    #[test]
    fn exhaustive() {
        for num in 0..=u16::MAX {
            let [a, b] = num.to_le_bytes();
            let bytes = twoten_from_parts(a, b);
            let _s = core::str::from_utf8(&bytes).unwrap();
            println!("{_s}");
        }
    }

    #[test]
    fn smoke() {
        #[rustfmt::skip]
        let checks = &[
            ("The sea was wet as wet could be,",    *b" AARON-236"),
            ("The sands were dry as dry.",          *b"SAMSON-335"),
            ("You could not see a cloud, because",  *b"RAHEEM-245"),
            ("No cloud was in the sky:",            *b" CLARA-326"),
            ("No birds were flying overhead —",     *b" FONDA-202"),
            ("There were no birds to fly.",         *b"URSINE-265"),
            ("The Walrus and the Carpenter",        *b" QUADE-271"),
            ("Were walking close at hand;",         *b"CANTON-151"),
            ("They wept like anything to see",      *b"  HALO-035"),
            ("Such quantities of sand:",            *b"PIETRO-065"),
            ("If this were only cleared away,",     *b"WILLOW-301"),
            ("They said, it would be grand!",       *b"VIOLET-315"),
        ];

        for (i, o) in checks {
            let out = twoten(i.as_bytes());
            println!("{i}");
            println!("{out:02X?} '{}'", core::str::from_utf8(&out).unwrap());
            assert_eq!(o, &out);
        }
    }
}
