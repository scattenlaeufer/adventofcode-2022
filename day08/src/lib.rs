#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<u8>>,
}

impl Map {
    pub fn from_input_vec(input: Vec<String>) -> Self {
        let mut grid = Vec::new();
        for line in input {
            let row = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            grid.push(row);
        }
        Map { grid }
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        self.grid[y][0..x]
            .iter()
            .map(|h| h < &self.grid[y][x])
            .all(|v| v)
            || self.grid[y][x + 1..self.grid[y].len()]
                .iter()
                .map(|h| h < &self.grid[y][x])
                .all(|v| v)
            || self.grid[y + 1..self.grid.len()]
                .iter()
                .map(|r| r[x] < self.grid[y][x])
                .all(|v| v)
            || self.grid[0..y]
                .iter()
                .map(|r| r[x] < self.grid[y][x])
                .all(|v| v)
    }

    pub fn get_number_of_visible_trees(&self) -> u32 {
        (self.grid.first().unwrap().len()
            + self.grid.last().unwrap().len()
            + (self.grid.len() - 2) * 2) as u32
            + self.grid[1..self.grid.len() - 1]
                .iter()
                .enumerate()
                .map(|(y, r)| {
                    r[1..r.len() - 1]
                        .iter()
                        .enumerate()
                        .map(|(x, _)| self.is_tree_visible(x + 1, y + 1))
                        .collect::<Vec<bool>>()
                })
                .flatten()
                .map(|v| v as u32)
                .sum::<u32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map() {
        let test_input: Vec<String> = vec![
            "30373".into(),
            "25512".into(),
            "65332".into(),
            "33549".into(),
            "35390".into(),
        ];
        println!("{:#?}", &test_input);
        let map = Map::from_input_vec(test_input);
        assert_eq!(true, map.is_tree_visible(1, 1));
        assert_eq!(true, map.is_tree_visible(2, 1));
        assert_eq!(false, map.is_tree_visible(3, 1));
        assert_eq!(true, map.is_tree_visible(1, 2));
        assert_eq!(false, map.is_tree_visible(2, 2));
        assert_eq!(true, map.is_tree_visible(3, 2));
        assert_eq!(false, map.is_tree_visible(1, 3));
        assert_eq!(true, map.is_tree_visible(2, 3));
        assert_eq!(false, map.is_tree_visible(3, 3));
        assert_eq!(21, map.get_number_of_visible_trees());
    }
}
