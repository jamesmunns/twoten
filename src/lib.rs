//! # twoten
//!
//! Hash data into two bytes, and turn it into an (exactly)
//! ten byte summary.
//!
//! Summaries are:
//!
//! * A 16-bit fnv1a hash of the data
//! * The first 8 bits of the hash are used to select a
//!     6-character word. See [`WORDS`].
//! * The next 8 bits of the hash are formatted in an octal format, `000` to `377`.
//!
//! The resulting summary looks like `JESTER-123`

use words::{decode, WORDS};

pub mod words;

pub fn twoten(data: &[u8]) -> [u8; 10] {
    // Hash the data, and fold it into two bytes
    let hash32 = fnv1a_32(data);
    let hash16 = hash32 ^ hash32 >> 16;
    let hash16 = (hash16 & 0xFFFF) as u16;
    let [name, oct] = hash16.to_le_bytes();

    // Lookup+decode the name.
    let name = WORDS[name as usize];
    let name = decode(name);
    // Decode the octal parts
    let octa = oct_bytes(oct);

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
    use crate::twoten;

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
            ("They wept like anything to see",      *b" HANOI-035"),
            ("Such quantities of sand:",            *b"PIETRO-065"),
            ("If this were only cleared away,",     *b"WILSON-301"),
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
