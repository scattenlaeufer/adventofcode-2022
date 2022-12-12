pub fn get_start_index(input: &str, n_chars: usize) -> usize {
    for i in 0..input.len() - n_chars {
        let sub_string = &input[i..i + n_chars];
        if sub_string
            .chars()
            .map(|c| sub_string.matches(c).count())
            .sum::<usize>()
            == n_chars
        {
            return i + n_chars;
        }
    }
    panic!("No matching sub string found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            7,
            get_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into(), 4)
        );
        assert_eq!(
            19,
            get_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into(), 14)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(5, get_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz".into(), 4));
        assert_eq!(
            23,
            get_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz".into(), 14)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(6, get_start_index("nppdvjthqldpwncqszvftbrmjlhg".into(), 4));
        assert_eq!(
            23,
            get_start_index("nppdvjthqldpwncqszvftbrmjlhg".into(), 14)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            10,
            get_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into(), 4)
        );
        assert_eq!(
            29,
            get_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into(), 14)
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            11,
            get_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into(), 4)
        );
        assert_eq!(
            26,
            get_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into(), 14)
        );
    }
}
