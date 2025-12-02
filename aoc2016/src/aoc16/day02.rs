use phf::phf_map;

const KEYPAD: [[&str; 3]; 3] = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];

static KEYPAD2: phf::Map<(u8, u8), &str> = phf_map! {
    (2u8,0u8) => "1",
    (1u8,1u8) => "2",
    (2u8,1u8) => "3",
    (3u8,1u8) => "4",
    (0u8,2u8) => "5",
    (1u8,2u8) => "6",
    (2u8,2u8) => "7",
    (3u8,2u8) => "8",
    (4u8,2u8) => "9",
    (1u8,3u8) => "A",
    (2u8,3u8) => "B",
    (3u8,3u8) => "C",
    (2u8,4u8) => "D"
};

pub fn part1(input: String) -> String {
    let mut code = String::default();
    let mut loc = (1usize, 1usize);
    for line in input.lines() {
        for c in line.as_bytes() {
            match c {
                b'U' => loc.1 = loc.1.saturating_sub(1),
                b'D' => loc.1 = (loc.1 + 1).min(2),
                b'L' => loc.0 = loc.0.saturating_sub(1),
                b'R' => loc.0 = (loc.0 + 1).min(2),
                _ => unimplemented!(),
            };
        }
        code += KEYPAD[loc.1][loc.0];
    }
    code
}
pub fn part2(input: String) -> String {
    let mut code = String::default();
    let mut loc = (0u8, 2u8);
    for line in input.lines() {
        for c in line.as_bytes() {
            let new_loc = match c {
                b'U' => (loc.0, loc.1.saturating_sub(1)),
                b'D' => (loc.0, (loc.1 + 1).min(4)),
                b'L' => (loc.0.saturating_sub(1), loc.1),
                b'R' => ((loc.0 + 1).min(4), loc.1),
                _ => unimplemented!(),
            };
            if KEYPAD2.contains_key(&new_loc) {
                loc = new_loc;
            }
        }
        code += KEYPAD2[&loc];
    }
    code
}
