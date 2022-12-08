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
        let wrong_item = self.get_wrong_item();
        if wrong_item.is_lowercase() {
            self.get_wrong_item() as u32 - 96
        } else {
            self.get_wrong_item() as u32 - 64 + 26
        }
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
}
