use std::fs;

pub fn read_input(day: u8, test: bool) -> String {
    let suffix = if test { "-test" } else { "" };
    let path = format!("inputs/day_{:02}{}.txt", day, suffix);
    match fs::read_to_string(&path) {
        Ok(input) => input,
        Err(err) => panic!("Error reading input file. path={} err={}", path, err),
    }
}
