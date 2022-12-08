use aoc_2022::utils::read_input;

const MAX_SIZE: usize = 100_000;
const DISK_SPACE: usize = 70_000_000;
const REQUIRED_SPACE: usize = 30_000_000;

fn solve(input: String) -> (usize, usize) {
    let mut stack = Vec::<(&str, usize)>::new();
    let mut fs = Vec::<usize>::new();

    let part_1 = input
        .split("$")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (cmd, output) = match s.trim().split_once("\n") {
                Some((cmd, output)) => (cmd, Some(output)),
                None => (s, None),
            };

            let (cmd, name) = cmd.trim().split_at(2);

            if cmd == "ls" {
                stack.last_mut().unwrap().1 += output
                    .unwrap()
                    .split("\n")
                    .map(|s| s.split_once(" ").unwrap().0.parse::<usize>().unwrap_or(0))
                    .sum::<usize>();
            } else if name.trim() != ".." {
                stack.push((name, 0));
            } else {
                let size = stack.pop().unwrap().1;
                stack.last_mut().unwrap().1 += size;
                fs.push(size);
                if size <= MAX_SIZE {
                    return size;
                }
            }

            return 0;
        })
        .sum();

    while stack.len() > 0 {
        let (_, size) = stack.pop().unwrap();
        fs.push(size);

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += size
        }
    }

    let required = REQUIRED_SPACE - (DISK_SPACE - *fs.last().unwrap());
    let part_2 = fs.into_iter().filter(|s| s >= &required).min().unwrap();

    return (part_1, part_2);
}

fn main() {
    let (part_1, part_2) = solve(read_input(7, false));
    println!("result[1]: {:?}", part_1);
    println!("result[2]: {:?}", part_2);
}
