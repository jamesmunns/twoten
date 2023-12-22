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
    w2w(*b" AARON"),
    w2w(*b"  ADAM"),
    w2w(*b"  ALEX"),
    w2w(*b" ALICE"),
    w2w(*b"ASTRAL"),
    w2w(*b"ATHENS"),
    w2w(*b"AUGUST"),
    w2w(*b"AUSTIN"),
    w2w(*b"   AVA"),
    w2w(*b"  AXEL"),
    // 10..20: B
    w2w(*b"BAILEY"),
    w2w(*b" BAKER"),
    w2w(*b"BEIRUT"),
    w2w(*b" BENNY"),
    w2w(*b"BENTON"),
    w2w(*b"BERLIN"),
    w2w(*b"BIANCA"),
    w2w(*b"BONNIE"),
    w2w(*b"BOSTON"),
    w2w(*b" BRIAR"),
    // 20..30: C
    w2w(*b"CANTON"),
    w2w(*b"CARMEL"),
    w2w(*b"CARTER"),
    w2w(*b" CLARA"),
    w2w(*b"CLOVER"),
    w2w(*b" CONAN"),
    w2w(*b"CORMAC"),
    w2w(*b" COSMO"),
    w2w(*b"  CUBA"),
    w2w(*b" CURIE"),
    // 30..40: D
    w2w(*b" DAISY"),
    w2w(*b"DANIEL"),
    w2w(*b" DANTE"),
    w2w(*b"DAPHNE"),
    w2w(*b"DECKER"),
    w2w(*b" DEREK"),
    w2w(*b" DIXIE"),
    w2w(*b"  DORA"),
    w2w(*b"DOUGAL"),
    w2w(*b"DUSTIN"),
    // 40..50: E
    w2w(*b"  EDEN"),
    w2w(*b"EDWARD"),
    w2w(*b"EILEEN"),
    w2w(*b" ELISE"),
    w2w(*b" ELTON"),
    w2w(*b" EMBER"),
    w2w(*b"  EMIL"),
    w2w(*b"  EMMA"),
    w2w(*b"ERHART"),
    w2w(*b"  ERIC"),
    // 50..60: F
    w2w(*b" FALCO"),
    w2w(*b"FENNEC"),
    w2w(*b"FERRIS"),
    w2w(*b"FENTON"),
    w2w(*b" FONDA"),
    w2w(*b"FRANKY"),
    w2w(*b"FRAZER"),
    w2w(*b" FREJA"),
    w2w(*b"FRISCO"),
    w2w(*b"FUZHOU"),
    // 60..70: G
    w2w(*b" GABBY"),
    w2w(*b"GALWAY"),
    w2w(*b"GAMBLE"),
    w2w(*b"GARDEN"),
    w2w(*b" GENIE"),
    w2w(*b"GERRIT"),
    w2w(*b"GLINDA"),
    w2w(*b"GORDON"),
    w2w(*b" GUIDO"),
    w2w(*b"GUSTAV"),
    // 70..80: H
    w2w(*b"HAMLET"),
    w2w(*b"HANNAH"),
    w2w(*b" HANOI"),
    w2w(*b"HARIET"),
    w2w(*b"HARLEM"),
    w2w(*b"HAROLD"),
    w2w(*b"HAWKER"),
    w2w(*b" HELEN"),
    w2w(*b" HENRY"),
    w2w(*b"HOPPER"),
    // 80..90: I
    w2w(*b"   IAN"),
    w2w(*b" IDRIS"),
    w2w(*b"  IGOR"),
    w2w(*b"INGRID"),
    w2w(*b"  IRMA"),
    w2w(*b"IRVING"),
    w2w(*b" ISAAC"),
    w2w(*b"  IVAN"),
    w2w(*b"   IVY"),
    w2w(*b" IZZIE"),
    // 90..100: J
    w2w(*b"JACQUI"),
    w2w(*b"JARVIS"),
    w2w(*b"JASMIN"),
    w2w(*b"JESTER"),
    w2w(*b"JASPER"),
    w2w(*b"JEFFRY"),
    w2w(*b"JETHRO"),
    w2w(*b" JEWEL"),
    w2w(*b"JORDAN"),
    w2w(*b"JUAREZ"),
    // 100..110: K
    w2w(*b" KATIE"),
    w2w(*b"KEEGAN"),
    w2w(*b" KEIRA"),
    w2w(*b"KENDRA"),
    w2w(*b" KEVIN"),
    w2w(*b" KIRBY"),
    w2w(*b"  KOBE"),
    w2w(*b"KRAKOW"),
    w2w(*b"  KYLE"),
    w2w(*b" KYOTO"),
    // 110..120: L
    w2w(*b" LACIE"),
    w2w(*b" LAURA"),
    w2w(*b" LAYLA"),
    w2w(*b"LENNON"),
    w2w(*b"LENORE"),
    w2w(*b" LILLY"),
    w2w(*b"LONDON"),
    w2w(*b" LOTTE"),
    w2w(*b" LOUIS"),
    w2w(*b" LUCKY"),
    // 120..130: M
    w2w(*b"MAISIE"),
    w2w(*b"MARCUS"),
    w2w(*b"MATTEO"),
    w2w(*b"MIGUEL"),
    w2w(*b" MILAN"),
    w2w(*b"MILLER"),
    w2w(*b"MILLIE"),
    w2w(*b" MOLLY"),
    w2w(*b"MORGAN"),
    w2w(*b"MURPHY"),
    // 130..140: N
    w2w(*b"NATHAN"),
    w2w(*b"NICOLE"),
    w2w(*b"NELLIE"),
    w2w(*b" NASIR"),
    w2w(*b" NOLAN"),
    w2w(*b"NORRIS"),
    w2w(*b"NEWTON"),
    w2w(*b"NELSON"),
    w2w(*b"NORMAN"),
    w2w(*b"NAPLES"),
    // 140..150: O
    w2w(*b" OLIVE"),
    w2w(*b"OSWALD"),
    w2w(*b"OCTAVE"),
    w2w(*b" ORION"),
    w2w(*b"OAKLEY"),
    w2w(*b" OSCAR"),
    w2w(*b"OSIRIS"),
    w2w(*b" OSCAR"),
    w2w(*b"OXNARD"),
    w2w(*b" OSAKA"),
    // 150..160: P
    w2w(*b"PHOEBE"),
    w2w(*b"PORTER"),
    w2w(*b"PIERRE"),
    w2w(*b"PATTON"),
    w2w(*b"PASCAL"),
    w2w(*b"PIETRO"),
    w2w(*b"PAWNEE"),
    w2w(*b"PINKIE"),
    w2w(*b"PANCHO"),
    w2w(*b"PIPPIN"),
    // 160..170: Q
    w2w(*b"QUINCY"),
    w2w(*b" QUADE"),
    w2w(*b"QUIGON"),
    w2w(*b" QUIET"),
    w2w(*b"QUEBEC"),
    w2w(*b" QUERY"),
    w2w(*b"QUINOA"),
    w2w(*b"QUARRY"),
    w2w(*b"QUIRKY"),
    w2w(*b"QUIVER"),
    // 170..180: R
    w2w(*b"ROLAND"),
    w2w(*b"RIPLEY"),
    w2w(*b"REUBEN"),
    w2w(*b"RUTHIE"),
    w2w(*b"RAMSES"),
    w2w(*b"RAHEEM"),
    w2w(*b"ROONEY"),
    w2w(*b"RAINEY"),
    w2w(*b"ROBERT"),
    w2w(*b"ROSLYN"),
    // 180..190: S
    w2w(*b"SOPHIE"),
    w2w(*b"SAMUEL"),
    w2w(*b"STELLA"),
    w2w(*b"SAWYER"),
    w2w(*b"SUMMER"),
    w2w(*b"STEVIE"),
    w2w(*b"SHELBY"),
    w2w(*b"SAMSON"),
    w2w(*b"STEFAN"),
    w2w(*b"SEAMUS"),
    // 190..200: T
    w2w(*b"THOMAS"),
    w2w(*b"TAYLOR"),
    w2w(*b"TRAVIS"),
    w2w(*b"TRUMAN"),
    w2w(*b"TYRONE"),
    w2w(*b"TINKER"),
    w2w(*b"THELMA"),
    w2w(*b"TAIPEI"),
    w2w(*b"TEHRAN"),
    w2w(*b"TUCSON"),
    // 200..210: U
    w2w(*b"UNIQUE"),
    w2w(*b"URBANE"),
    w2w(*b"ULRICH"),
    w2w(*b"UPTOWN"),
    w2w(*b" UMBER"),
    w2w(*b" UNION"),
    w2w(*b"ULSTER"),
    w2w(*b"UPBEAT"),
    w2w(*b"URSINE"),
    w2w(*b"URGENT"),
    // 210..220: V
    w2w(*b"VIOLET"),
    w2w(*b"VICTOR"),
    w2w(*b"VERNON"),
    w2w(*b"VENICE"),
    w2w(*b" VELMA"),
    w2w(*b"VIRGIL"),
    w2w(*b"VISION"),
    w2w(*b" VISTA"),
    w2w(*b" VALOR"),
    w2w(*b"VESPER"),
    // 220..230: W
    w2w(*b"WILLOW"),
    w2w(*b"WESLEY"),
    w2w(*b"WINNIE"),
    w2w(*b"WILSON"),
    w2w(*b" WANDA"),
    w2w(*b"WORTHY"),
    w2w(*b"WERNER"),
    w2w(*b"WINTON"),
    w2w(*b"WALDER"),
    w2w(*b"WARSAW"),
    // 230..240: X
    w2w(*b"XAVIER"),
    w2w(*b"XANDER"),
    w2w(*b"XERXES"),
    w2w(*b"  XYLA"),
    w2w(*b" XENON"),
    w2w(*b" XENIA"),
    w2w(*b"  XIAO"),
    w2w(*b"XENIAL"),
    w2w(*b"  XRAY"),
    w2w(*b"  XIPH"),
    // 240..250: Y
    w2w(*b"YOUSEF"),
    w2w(*b"YVETTE"),
    w2w(*b" YOSHI"),
    w2w(*b"YACHTY"),
    w2w(*b"YEOMAN"),
    w2w(*b"YELLOW"),
    w2w(*b"YELPER"),
    w2w(*b"YIPPEE"),
    w2w(*b"YOGURT"),
    w2w(*b"YEARLY"),
    // 250..256: Z
    w2w(*b" ZELDA"),
    w2w(*b"ZAGREB"),
    w2w(*b"ZATAAR"),
    w2w(*b"ZEALOT"),
    w2w(*b"ZIGZAG"),
    w2w(*b"ZEPHYR"),
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
