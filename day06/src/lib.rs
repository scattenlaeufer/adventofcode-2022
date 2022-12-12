pub fn get_start_index(input: &str) -> usize {
    for i in 0..input.len() - 4 {
        let sub_string = &input[i..i + 4];
        if sub_string
            .chars()
            .map(|c| sub_string.matches(c).count())
            .sum::<usize>()
            == 4
        {
            return i + 4;
        }
    }
    panic!("No matching sub string found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, get_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into()))
    }

    #[test]
    fn test_2() {
        assert_eq!(5, get_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz".into()))
    }

    #[test]
    fn test_3() {
        assert_eq!(6, get_start_index("nppdvjthqldpwncqszvftbrmjlhg".into()))
    }

    #[test]
    fn test_4() {
        assert_eq!(
            10,
            get_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into())
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            11,
            get_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into())
        )
    }
}
