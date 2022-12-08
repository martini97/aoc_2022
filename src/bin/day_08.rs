use std::collections::HashMap;

use aoc_2022::utils::read_input;

macro_rules! skip_tree_edges {
    ($row: expr, $col: expr, $row_count: ident, $col_count: ident, $default: expr) => {
        if $row.min($col) == 0 || $row + 1 == $row_count || $col + 1 == $col_count {
            return $default;
        }
    };
}

fn main() {
    let input = read_input(8, false);

    let row_count = input.trim().lines().count();
    let col_count = input.trim().lines().next().unwrap().chars().count();

    let mut rows = HashMap::new();
    let mut cols = HashMap::new();

    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .for_each(|(row, col, c)| {
            let tree = c.to_digit(10).unwrap();
            rows.entry(row).or_insert(vec![0; row_count])[col] = tree;
            cols.entry(col).or_insert(vec![0; col_count])[row] = tree;
        });

    let part_1 = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(r, c, t)| {
            skip_tree_edges!(*r, *c, row_count, col_count, true);

            let tree = &t.to_digit(10).unwrap();
            let row = rows.get(r).unwrap();
            let col = cols.get(c).unwrap();
            let is_visible = |v: Vec<u32>| v.iter().max().unwrap_or(&0).lt(tree);

            return is_visible(row[0..*c].to_vec())
                || is_visible(row[c + 1..].to_vec())
                || is_visible(col[0..*r].to_vec())
                || is_visible(col[r + 1..].to_vec());
        })
        .count();

    let part_2 = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .map(|(r, c, t)| {
            skip_tree_edges!(r, c, row_count, col_count, 0);

            let tree = t.to_digit(10).unwrap();
            let row = rows.get(&r).unwrap();
            let col = cols.get(&c).unwrap();

            let backward = |v: Vec<u32>| {
                v.iter()
                    .rev()
                    .position(|&l| l.ge(&tree))
                    .unwrap_or(v.len() - 1)
                    + 1
            };

            let forward =
                |v: Vec<u32>| v.iter().position(|&l| l.ge(&tree)).unwrap_or(v.len() - 1) + 1;

            let right = forward(row[c + 1..].to_vec());
            let top = backward(col[0..r].to_vec());
            let left = backward(row[0..c].to_vec());
            let bottom = forward(col[r + 1..].to_vec());

            return left * right * top * bottom;
        })
        .max()
        .unwrap();

    println!("part_1: {:?}", part_1);
    println!("part_2: {:?}", part_2);
}
