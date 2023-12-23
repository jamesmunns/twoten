/// Words!
///
/// Words selected by @jamesmunns. General rules:
///
/// 1. Words MUST be exactly two syllables.
/// 2. Words MUST be <= 6 ascii characters, in upper case
/// 3. Words MUST be right aligned
/// 4. Prefer words that are longer rather than shorter. Ideally all should be 6 letters.
/// 5. Prefer names over places over other.
/// 6. When in doubt: will it make you smile when you see it in a log file?
pub const WORDS: &[u32; 256] = &[
    // 0..10: A
    w2w(*b"  ALEX"),
    w2w(*b"  AXEL"),
    w2w(*b" AARON"),
    w2w(*b"ADAINE"),
    w2w(*b"ALFRED"),
    w2w(*b"ASTRAL"),
    w2w(*b"ASTRID"),
    w2w(*b"ATHENS"),
    w2w(*b"AUGUST"),
    w2w(*b"AUSTIN"),
    // 10..20: B
    w2w(*b" BAKER"),
    w2w(*b" BENNY"),
    w2w(*b" BRIAR"),
    w2w(*b"BAILEY"),
    w2w(*b"BEIRUT"),
    w2w(*b"BENTON"),
    w2w(*b"BERLIN"),
    w2w(*b"BIANCA"),
    w2w(*b"BONNIE"),
    w2w(*b"BOSTON"),
    // 20..30: C
    w2w(*b" CLARA"),
    w2w(*b" CONAN"),
    w2w(*b" COSMO"),
    w2w(*b" CURIE"),
    w2w(*b"CANTON"),
    w2w(*b"CARMEL"),
    w2w(*b"CARTER"),
    w2w(*b"CASPER"),
    w2w(*b"CLOVER"),
    w2w(*b"CORMAC"),
    // 30..40: D
    w2w(*b" DAISY"),
    w2w(*b" DANTE"),
    w2w(*b" DEREK"),
    w2w(*b" DIXIE"),
    w2w(*b"DANIEL"),
    w2w(*b"DAPHNE"),
    w2w(*b"DECKER"),
    w2w(*b"DEXTER"),
    w2w(*b"DOUGAL"),
    w2w(*b"DUSTIN"),
    // 40..50: E
    w2w(*b"  EDEN"),
    w2w(*b"  EMIL"), // Too close?
    w2w(*b"  EMMA"), // Emma + Emil?
    w2w(*b"  ERIC"),
    w2w(*b" ELISE"),
    w2w(*b" ELTON"),
    w2w(*b" EMBER"),
    w2w(*b"EDWARD"),
    w2w(*b"EILEEN"),
    w2w(*b"ERHART"),
    // 50..60: F
    w2w(*b" FALCO"),
    w2w(*b" FONDA"),
    w2w(*b" FREJA"),
    w2w(*b"FENNEC"),
    w2w(*b"FENTON"),
    w2w(*b"FERRIS"),
    w2w(*b"FRANKY"),
    w2w(*b"FRAZER"),
    w2w(*b"FRISCO"),
    w2w(*b"FUZHOU"),
    // 60..70: G
    w2w(*b" GABBY"),
    w2w(*b" GENIE"),
    w2w(*b" GUIDO"),
    w2w(*b"GALWAY"),
    w2w(*b"GAMBLE"),
    w2w(*b"GARDEN"),
    w2w(*b"GERRIT"),
    w2w(*b"GLINDA"),
    w2w(*b"GORDON"),
    w2w(*b"GUSTAV"),
    // 70..80: H
    w2w(*b" HANOI"),
    w2w(*b" HELEN"),
    w2w(*b"HAMLET"),
    w2w(*b"HANNAH"),
    w2w(*b"HARIET"), // lotta "har..."
    w2w(*b"HARLEM"), // lotta "har..."
    w2w(*b"HAROLD"), // lotta "har..."
    w2w(*b"HARVEY"), // lotta "har..."
    w2w(*b"HAWKER"),
    w2w(*b"HOPPER"),
    // 80..90: I
    w2w(*b"   IAN"),
    w2w(*b"   IVY"),
    w2w(*b"  IGOR"),
    w2w(*b"  IRMA"),
    w2w(*b"  IVAN"), // Probably too close to IAN + IVY
    w2w(*b" IDRIS"),
    w2w(*b" ISAAC"),
    w2w(*b" IZZIE"),
    w2w(*b"INGRID"),
    w2w(*b"IRVING"),
    // 90..100: J
    w2w(*b"JACQUI"),
    w2w(*b"JARVIS"),
    w2w(*b"JASMIN"), // too close?
    w2w(*b"JASPER"), // too close?
    w2w(*b"JEFFRY"),
    w2w(*b"JESTER"),
    w2w(*b"JETHRO"),
    w2w(*b"JOLENE"),
    w2w(*b"JORDAN"),
    w2w(*b"JUAREZ"),
    // 100..110: K
    w2w(*b"  KOBE"),
    w2w(*b"  KYLE"),
    w2w(*b" KATIE"),
    w2w(*b" KEIRA"),
    w2w(*b" KEVIN"),
    w2w(*b" KIRBY"),
    w2w(*b" KYOTO"),
    w2w(*b"KEEGAN"),
    w2w(*b"KENDRA"),
    w2w(*b"KRAKOW"),
    // 110..120: L
    w2w(*b" LACIE"),
    w2w(*b" LAURA"),
    w2w(*b" LAYLA"),
    w2w(*b" LILLY"),
    w2w(*b" LOTTE"),
    w2w(*b" LOUIS"),
    w2w(*b" LUCKY"),
    w2w(*b"LENNON"), // TOO CLOSE? Lennon + Lenore?
    w2w(*b"LENORE"), // TOO CLOSE? Lennon + Lenore?
    w2w(*b"LONDON"),
    // 120..130: M
    w2w(*b"MAGLEV"),
    w2w(*b"MAGNUM"),
    w2w(*b"MAISIE"),
    w2w(*b"MARCUS"),
    w2w(*b"MATCHA"),
    w2w(*b"MICKEY"),
    w2w(*b"MIGUEL"),
    w2w(*b"MILLIE"),
    w2w(*b"MORGAN"),
    w2w(*b"MURPHY"),
    // 130..140: N
    w2w(*b"NAPLES"),
    w2w(*b"NATHAN"),
    w2w(*b"NELLIE"),
    w2w(*b"NEWTON"),
    w2w(*b"NIBBLE"),
    w2w(*b"NICOLE"),
    w2w(*b"NORMAN"),
    w2w(*b"NOTICE"),
    w2w(*b"NOZZLE"),
    w2w(*b"NUTMEG"),
    // 140..150: O
    w2w(*b"OAKLEY"),
    w2w(*b"OBJECT"),
    w2w(*b"OCCULT"),
    w2w(*b"OCTAVE"),
    w2w(*b"OFFSET"),
    w2w(*b"ONRUSH"),
    w2w(*b"OOMPAH"),
    w2w(*b"OPAQUE"),
    w2w(*b"OSWALD"),
    w2w(*b"OXNARD"),
    // 150..160: P
    w2w(*b"PANCHO"),
    w2w(*b"PASCAL"),
    w2w(*b"PATTON"),
    w2w(*b"PAWNEE"),
    w2w(*b"PHOEBE"),
    w2w(*b"PIERRE"),
    w2w(*b"PIETRO"),
    w2w(*b"PINKIE"),
    w2w(*b"PIPPIN"),
    w2w(*b"PORTER"),
    // 160..170: Q
    w2w(*b" QUADE"),
    w2w(*b" QUERY"),
    w2w(*b" QUIET"), // lotta "qui..."
    w2w(*b"QUARRY"),
    w2w(*b"QUEBEC"),
    w2w(*b"QUIGON"), // lotta "qui..."
    w2w(*b"QUINCY"), // lotta "qui..."
    w2w(*b"QUINOA"), // lotta "qui..."
    w2w(*b"QUIRKY"), // lotta "qui..."
    w2w(*b"QUIVER"), // lotta "qui..."
    // 170..180: R
    w2w(*b"RAHEEM"),
    w2w(*b"RAINEY"),
    w2w(*b"RAMSES"),
    w2w(*b"REUBEN"),
    w2w(*b"RIPLEY"),
    w2w(*b"ROBERT"),
    w2w(*b"ROLAND"),
    w2w(*b"ROONEY"),
    w2w(*b"ROSLYN"),
    w2w(*b"RUTHIE"),
    // 180..190: S
    w2w(*b"SAMSON"),
    w2w(*b"SAMUEL"),
    w2w(*b"SAWYER"),
    w2w(*b"SEAMUS"),
    w2w(*b"SHELBY"),
    w2w(*b"SOPHIE"),
    w2w(*b"STEFAN"),
    w2w(*b"STELLA"),
    w2w(*b"STEVIE"),
    w2w(*b"SUMMER"),
    // 190..200: T
    w2w(*b"TAIPEI"),
    w2w(*b"TAYLOR"),
    w2w(*b"TEHRAN"),
    w2w(*b"THELMA"),
    w2w(*b"THOMAS"),
    w2w(*b"TINKER"),
    w2w(*b"TRAVIS"),
    w2w(*b"TRUMAN"),
    w2w(*b"TUCSON"),
    w2w(*b"TYRONE"),
    // 200..210: U
    w2w(*b" UMBER"),
    w2w(*b" UNION"),
    w2w(*b"ULRICH"),
    w2w(*b"ULSTER"),
    w2w(*b"UNIQUE"),
    w2w(*b"UPBEAT"),
    w2w(*b"UPTOWN"),
    w2w(*b"URBANE"),
    w2w(*b"URGENT"),
    w2w(*b"URSINE"),
    // 210..220: V
    w2w(*b" VALOR"),
    w2w(*b" VELMA"),
    w2w(*b" VISTA"),
    w2w(*b"VENICE"),
    w2w(*b"VERNON"),
    w2w(*b"VESPER"),
    w2w(*b"VICTOR"),
    w2w(*b"VIOLET"),
    w2w(*b"VIRGIL"),
    w2w(*b"VISION"),
    // 220..230: W
    w2w(*b" WANDA"),
    w2w(*b"WALDER"),
    w2w(*b"WARSAW"),
    w2w(*b"WERNER"),
    w2w(*b"WESLEY"),
    w2w(*b"WILLOW"), // wil... too close?
    w2w(*b"WILSON"), // wil... too close?
    w2w(*b"WINNIE"), // win... too close?
    w2w(*b"WINTON"), // win... too close?
    w2w(*b"WORTHY"),
    // 230..240: Z
    //
    // note: q was the hardest, so it's the last 6
    w2w(*b" ZELDA"),
    w2w(*b"ZAGREB"),
    w2w(*b"ZATAAR"),
    w2w(*b"ZEALOT"),
    w2w(*b"ZEBRAS"),
    w2w(*b"ZEPHYR"),
    w2w(*b"ZEROES"),
    w2w(*b"ZIGZAG"),
    w2w(*b"ZIRCON"),
    w2w(*b"ZYGOTE"),
    // 240..250: Y
    w2w(*b" YOSHI"),
    w2w(*b"YACHTY"),
    w2w(*b"YEARLY"),
    w2w(*b"YELLOW"),
    w2w(*b"YELPER"),
    w2w(*b"YEOMAN"),
    w2w(*b"YIPPEE"),
    w2w(*b"YOGURT"),
    w2w(*b"YOUSEF"),
    w2w(*b"YVETTE"),
    // 250..256: X
    w2w(*b"  XRAY"),
    w2w(*b"  XYLA"),
    w2w(*b" XENON"),
    w2w(*b"XANDER"),
    w2w(*b"XAVIER"),
    w2w(*b"XERXES"),
];

/// Take a 6 char, all caps ascii word, to a packed
/// [`u32`].
const fn w2w(val: [u8; 6]) -> u32 {
    let mut out = 0u32;
    let mut idx = 0;
    while idx < 6 {
        out <<= 5;
        let val = match val[idx] {
            v @ b'A'..=b'Z' => v - b'A',
            b' ' => 0b11111,
            _ => unreachable!(),
        };
        out |= val as u32;
        idx += 1;
    }
    out
}

/// Decode a packed word to 6 ascii bytes
pub fn decode(mut v: u32) -> [u8; 6] {
    let mut out = [0u8; 6];
    out.iter_mut().rev().for_each(|b| {
        let ch = (v & 0b11111) as u8;
        *b = if ch > 25 {
            b' '
        } else {
            ch + b'A'
        };
        v >>= 5;
    });
    out
}
