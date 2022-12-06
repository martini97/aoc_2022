use std::collections::HashSet;

pub fn get_first_marker_at(datastream: &str, distinct: usize) -> usize {
    for (i, window) in datastream
        .chars()
        .collect::<Vec<char>>()
        .windows(distinct)
        .enumerate()
    {
        if HashSet::<char>::from_iter(window.to_vec()).len() == distinct {
            return i + distinct;
        }
    }
    unreachable!("what");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_marker_at() {
        assert_eq!(get_first_marker_at("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(get_first_marker_at("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_first_marker_at("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            get_first_marker_at("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            get_first_marker_at("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );

        assert_eq!(
            get_first_marker_at("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(get_first_marker_at("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(get_first_marker_at("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            get_first_marker_at("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            get_first_marker_at("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
