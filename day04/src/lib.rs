#[derive(Debug)]
pub struct Group {
    one: Vec<u32>,
    two: Vec<u32>,
}

impl Group {
    pub fn from_str(input: &str) -> Self {
        let bounds = input
            .split(',')
            .map(|v| {
                v.split('-')
                    .map(|b| b.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let one = (bounds[0][0]..bounds[0][1] + 1).collect();
        let two = (bounds[1][0]..bounds[1][1] + 1).collect();
        Group { one, two }
    }

    pub fn one_contains_the_other(&self) -> bool {
        self.one.iter().all(|o| self.two.contains(o))
            || self.two.iter().all(|t| self.one.contains(t))
    }

    pub fn they_overlap(&self) -> bool {
        self.one.iter().any(|o| self.two.contains(o))
            || self.two.iter().any(|t| self.one.contains(t))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_group(input: &str, contain: bool, overlap: bool) {
        let group = Group::from_str(input);
        println!("{:#?}", &group);
        assert_eq!(contain, group.one_contains_the_other());
        assert_eq!(overlap, group.they_overlap());
    }

    #[test]
    fn group_1() {
        test_group("2-4,6-8", false, false);
    }

    #[test]
    fn group_2() {
        test_group("2-3,4-5", false, false);
    }

    #[test]
    fn group_3() {
        test_group("5-7,7-9", false, true);
    }

    #[test]
    fn group_4() {
        test_group("2-8,3-7", true, true);
    }

    #[test]
    fn group_5() {
        test_group("6-6,4-6", true, true);
    }

    #[test]
    fn group_6() {
        test_group("2-6,4-8", false, true);
    }
}
