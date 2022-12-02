pub fn parse_input(input: &str) -> Vec<usize> {
    return input
        .split("\n\n")
        .map(|e| e.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect::<Vec<usize>>();
}
