use aoc2022::timing;
use std::{cmp::Ord, collections::HashMap};

#[derive(Debug)]
struct Root {
    dirs: Vec<Dir>,
}

#[derive(Debug)]
struct Dir {
    level: Option<usize>,
    subdirs: HashMap<&'static str, usize>,
    size: usize,
}

impl Dir {
    fn new(level: Option<usize>) -> Self {
        Self {
            level,
            subdirs: HashMap::new(),
            size: 0,
        }
    }
}

impl Dir {
    fn total(&self, dirs: &[Self]) -> usize {
        let subdirs_size: usize = self
            .subdirs
            .values()
            .map(|&idx| dirs[idx].total(dirs))
            .sum();

        subdirs_size + self.size
    }
}

impl From<&'static str> for Root {
    fn from(input: &'static str) -> Self {
        let mut dirs = vec![Dir::new(Some(0))];
        let mut current_level = 0;

        for line in input.lines() {
            match line {
                "$ cd /" => current_level = 0,
                "$ cd .." => current_level = dirs[current_level].level.unwrap(),
                "$ ls" => (),
                _ if line.starts_with("$ cd ") => {
                    current_level =
                        dirs[current_level].subdirs[line.strip_prefix("$ cd ").unwrap()];
                }
                _ if line.starts_with("dir ") => {
                    let dir = dirs.len();
                    dirs.push(Dir::new(Some(current_level)));
                    dirs[current_level]
                        .subdirs
                        .insert(line.strip_prefix("dir ").unwrap(), dir);
                }
                _ => {
                    let (size, _) = line.split_once(' ').unwrap();
                    dirs[current_level].size += size.parse::<usize>().unwrap();
                }
            }
        }
        Self { dirs }
    }
}

fn part_one(input: &'static str) -> usize {
    let at_least = 100_000;
    let filesystem = Root::from(input);
    filesystem
        .dirs
        .iter()
        .filter_map(|dir| {
            let size = dir.total(filesystem.dirs.as_slice());
            match size {
                s if s <= at_least => Some(s),
                _ => None,
            }
        })
        .sum()
}

fn part_two(input: &'static str) -> usize {
    let filesystem = Root::from(input);
    let need = filesystem.dirs[0].total(filesystem.dirs.as_slice()) - 40_000_000;
    filesystem
        .dirs
        .iter()
        .filter_map(|d| {
            let size = d.total(filesystem.dirs.as_slice());
            match size {
                s if s >= need => Some(s),
                _ => None,
            }
        })
        .fold(usize::MAX, Ord::min)
}

pub fn run() {
    let input = include_str!("../input/day7/input.txt");

    println!("DAY 7:");
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_part_one() {
        assert_eq!(95_437, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(24_933_642, part_two(INPUT));
    }
}
