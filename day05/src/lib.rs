#[derive(Debug)]
pub struct Plan {
    stacks: Stacks,
    directives: Vec<Directive>,
}

impl Plan {
    pub fn from_vec(input: Vec<String>) -> Self {
        let split = input.iter().position(|l| l == "").unwrap();

        Self {
            stacks: Stacks::from_vec(&input[0..split]),
            directives: input[split + 1..input.len()]
                .iter()
                .map(|d| Directive::from_str(&d))
                .collect(),
        }
    }

    pub fn run(&mut self) -> String {
        for directive in &self.directives {
            self.stacks.run_directive(&directive);
        }
        self.stacks
            .stack_vec
            .iter()
            .map(|s| s.last().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Debug)]
struct Stacks {
    stack_vec: Vec<Vec<char>>,
}

impl Stacks {
    fn from_vec(input: &[String]) -> Self {
        let n_stacks: usize = (input.last().unwrap().len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n_stacks);
        for _ in 0..n_stacks {
            stacks.push(Vec::new());
        }
        for line in input[0..input.len() - 1].iter().rev() {
            let chars = line.chars().collect::<Vec<char>>();
            for i in 0..n_stacks {
                let c = chars[i * 4 + 1];
                if !(c == ' ') {
                    stacks[i].push(c);
                }
            }
        }
        Stacks { stack_vec: stacks }
    }

    fn run_directive(&mut self, directive: &Directive) {
        for _ in 0..directive.amount {
            let item = self.stack_vec[directive.from - 1].pop().unwrap();
            self.stack_vec[directive.to - 1].push(item);
        }
    }
}

#[derive(Debug)]
struct Directive {
    amount: usize,
    from: usize,
    to: usize,
}

impl Directive {
    fn from_str(input: &str) -> Self {
        let input_vec = input.split(" ").collect::<Vec<&str>>();
        Self {
            amount: input_vec[1].parse().unwrap(),
            from: input_vec[3].parse().unwrap(),
            to: input_vec[5].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plan() {
        let test_input: Vec<String> = vec![
            "    [D]    ".into(),
            "[N] [C]    ".into(),
            "[Z] [M] [P]".into(),
            " 1   2   3 ".into(),
            "".into(),
            "move 1 from 2 to 1".into(),
            "move 3 from 1 to 3".into(),
            "move 2 from 2 to 1".into(),
            "move 1 from 1 to 2".into(),
        ];
        let mut plan = Plan::from_vec(test_input);
        println!("{:#?}", &plan);
        assert_eq!(vec!['Z', 'N'], plan.stacks.stack_vec[0]);
        assert_eq!(vec!['M', 'C', 'D'], plan.stacks.stack_vec[1]);
        assert_eq!(vec!['P'], plan.stacks.stack_vec[2]);

        assert_eq!("CMZ", plan.run());
    }
}
