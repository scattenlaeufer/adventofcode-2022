use std::collections::HashMap;

const FILE_SYSTEM_SIZE: usize = 70000000;
pub const MIN_FREE_SPACE: usize = 30000000;

#[derive(Debug)]
enum Content {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
pub struct Directory {
    content: HashMap<String, Content>,
}

impl<'b> Directory {
    pub fn init(print: Vec<String>) -> Self {
        let (_, mut print) = print.split_first().unwrap();
        let mut dir = Directory {
            content: HashMap::new(),
        };
        loop {
            print = dir.parse_print(print);
            if print.is_empty() {
                break;
            }
        }
        dir
    }

    fn parse_print<'a>(&mut self, print: &'a [String]) -> &'a [String] {
        let mut print_rest = print;
        loop {
            let (directive, print) = print_rest.split_first().unwrap();
            print_rest = print;
            let directive = directive.split(' ').collect::<Vec<&str>>();
            if directive[1] == "ls" {
                print_rest = self.ls(print_rest);
            } else if directive[2] == ".." {
                return print_rest;
            } else {
                match self.content.get_mut(directive[2]).unwrap() {
                    Content::File(_) => panic!("Tried to cd into file {:?}", directive[2]),
                    Content::Directory(d) => print_rest = d.parse_print(print_rest),
                };
            }
            if print_rest.is_empty() {
                return print_rest;
            }
        }
    }

    fn ls<'a: 'b>(&mut self, print: &'a [String]) -> &'a [String] {
        let mut ls_print = print;
        loop {
            if ls_print.is_empty() || ls_print[0].starts_with('$') {
                return ls_print;
            }
            let (file_desc, print) = ls_print.split_first().unwrap();
            ls_print = print;
            let file_desc = file_desc.split(' ').collect::<Vec<&str>>();
            let content = if file_desc[0] == "dir" {
                Content::Directory(Directory {
                    content: HashMap::new(),
                })
            } else {
                Content::File(File {
                    size: file_desc[0].parse().unwrap(),
                })
            };
            self.content.insert(file_desc[1].into(), content);
        }
    }

    fn get_dir_size(&self) -> usize {
        self.content
            .iter()
            .map(|(_, c)| match c {
                Content::File(f) => f.size,
                Content::Directory(d) => d.get_dir_size(),
            })
            .sum()
    }

    pub fn get_dir_size_below(&self, max_size: usize) -> usize {
        let mut self_size = self.get_dir_size();
        if self_size > max_size {
            self_size = 0;
        }
        self_size
            + self
                .content
                .iter()
                .map(|(_, c)| match c {
                    Content::File(_) => 0,
                    Content::Directory(d) => d.get_dir_size_below(max_size),
                })
                .sum::<usize>()
    }

    fn get_dir_size_vec(&self) -> Vec<usize> {
        let mut dir_size_vec = self
            .content
            .iter()
            .filter_map(|(_, c)| match c {
                Content::File(_) => None,
                Content::Directory(d) => Some(d.get_dir_size_vec()),
            })
            .flatten()
            .collect::<Vec<_>>();
        dir_size_vec.push(self.get_dir_size());
        dir_size_vec
    }

    pub fn get_smallest_dir_size_to_delete(&self, min_size: usize) -> usize {
        let required_space = min_size - (FILE_SYSTEM_SIZE - self.get_dir_size());
        let mut dir_size = self.get_dir_size_vec();
        dir_size.sort();
        *dir_size.iter().find(|s| *s > &required_space).unwrap()
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn directory_test() {
        let console_print: Vec<String> = vec![
            "$ cd /".into(),
            "$ ls".into(),
            "dir a".into(),
            "14848514 b.txt".into(),
            "8504156 c.dat".into(),
            "dir d".into(),
            "$ cd a".into(),
            "$ ls".into(),
            "dir e".into(),
            "29116 f".into(),
            "2557 g".into(),
            "62596 h.lst".into(),
            "$ cd e".into(),
            "$ ls".into(),
            "584 i".into(),
            "$ cd ..".into(),
            "$ cd ..".into(),
            "$ cd d".into(),
            "$ ls".into(),
            "4060174 j".into(),
            "8033020 d.log".into(),
            "5626152 d.ext".into(),
            "7214296 k".into(),
        ];
        let dir = Directory::init(console_print);
        assert_eq!(95437, dir.get_dir_size_below(100000));
        assert_eq!(
            24933642,
            dir.get_smallest_dir_size_to_delete(MIN_FREE_SPACE)
        );
    }
}
