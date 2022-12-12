#[derive(Debug)]
pub struct Rucksack {
    first: String,
    second: String,
}

impl Rucksack {
    pub fn from_content(content: &str) -> Self {
        let pivot = content.len() / 2;
        let first = content[0..pivot].to_string();
        let second = content[pivot..content.len()].to_string();
        Rucksack { first, second }
    }

    fn get_wrong_item(&self) -> char {
        for c in self.first.chars() {
            if self.second.contains(c) {
                return c;
            }
        }

        for c in self.second.chars() {
            if self.first.contains(c) {
                return c;
            }
        }

        panic!("Can't find a wrong item!");
    }

    pub fn get_priority(&self) -> u32 {
        get_priority(self.get_wrong_item())
    }
}

#[derive(Debug)]
pub struct Group<'a> {
    rucksacks: [&'a Rucksack; 3],
}

impl<'a> Group<'a> {
    pub fn new(rucksacks: [&'a Rucksack; 3]) -> Self {
        Self { rucksacks }
    }

    fn get_common_item_type(&'a self) -> char {
        for c in self.rucksacks[0].first.chars() {
            if (self.rucksacks[1].first.contains(c) || self.rucksacks[1].second.contains(c))
                && (self.rucksacks[2].first.contains(c) || self.rucksacks[2].second.contains(c))
            {
                return c;
            }
        }
        for c in self.rucksacks[0].second.chars() {
            if (self.rucksacks[1].first.contains(c) || self.rucksacks[1].second.contains(c))
                && (self.rucksacks[2].first.contains(c) || self.rucksacks[2].second.contains(c))
            {
                return c;
            }
        }
        panic!("No common item found: {:?}", self);
    }

    pub fn get_priority(&'a self) -> u32 {
        get_priority(self.get_common_item_type())
    }
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run_rucksack_test(
        content: &str,
        first: Option<&str>,
        second: Option<&str>,
        item: char,
        priority: u32,
    ) {
        let rucksack = Rucksack::from_content(content);
        if let Some(f) = first {
            assert_eq!(f, rucksack.first);
        }
        if let Some(s) = second {
            assert_eq!(s, rucksack.second);
        }
        assert_eq!(item, rucksack.get_wrong_item());
        assert_eq!(priority, rucksack.get_priority())
    }

    #[test]
    fn rucksack_1() {
        run_rucksack_test(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            Some("vJrwpWtwJgWr"),
            Some("hcsFMMfFFhFp"),
            'p',
            16,
        );
    }

    #[test]
    fn rucksack_2() {
        run_rucksack_test(
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            Some("jqHRNqRjqzjGDLGL"),
            Some("rsFMfFZSrLrFZsSL"),
            'L',
            38,
        );
    }

    #[test]
    fn rucksack_3() {
        run_rucksack_test(
            "PmmdzqPrVvPwwTWBwg",
            Some("PmmdzqPrV"),
            Some("vPwwTWBwg"),
            'P',
            42,
        );
    }

    #[test]
    fn rucksack_4() {
        run_rucksack_test("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", None, None, 'v', 22);
    }

    #[test]
    fn rucksack_5() {
        run_rucksack_test("ttgJtRGJQctTZtZT", None, None, 't', 20);
    }

    #[test]
    fn rucksack_6() {
        run_rucksack_test("CrZsJsPPZsGzwwsLwLmpwMDw", None, None, 's', 19);
    }

    fn run_group_test(contents: [&str; 3], common_item: char, priority: u32) {
        let r1 = Rucksack::from_content(contents[0]);
        let r2 = Rucksack::from_content(contents[1]);
        let r3 = Rucksack::from_content(contents[2]);
        let group = Group {
            rucksacks: [&r1, &r2, &r3],
        };
        assert_eq!(common_item, group.get_common_item_type());
        assert_eq!(priority, group.get_priority());
    }

    #[test]
    fn group_1() {
        run_group_test(
            [
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ],
            'r',
            18,
        );
    }

    #[test]
    fn group_2() {
        run_group_test(
            [
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
            'Z',
            52,
        );
    }
}
