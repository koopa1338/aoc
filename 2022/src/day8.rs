use std::ops::Index;

use aoc2022::timing;

struct TreeGrid<'a> {
    trees: &'a [u8],
    width: usize,
    height: usize,
    offset: usize,
}

type TreePosition = (usize, usize);

impl<'a> From<&'a str> for TreeGrid<'a> {
    fn from(input: &'a str) -> Self {
        let trees = input.as_bytes();
        let mut iter = input.lines();
        let height = iter.clone().count();
        let width = iter.next().unwrap().chars().count();
        Self {
            trees,
            width,
            height,
            offset: width + 1,
        }
    }
}

impl<'a> Index<TreePosition> for TreeGrid<'a> {
    type Output = u8;

    fn index(&self, index: TreePosition) -> &Self::Output {
        &self.trees[index.0 + self.offset * index.1]
    }
}

impl<'a> TreeGrid<'a> {
    fn tree_at(&self, pos: TreePosition) -> u8 {
        self.trees[pos.0 + self.offset * pos.1]
    }
    fn is_visible(&self, pos: TreePosition) -> bool {
        let tree_pos = self.tree_at(pos);
        (0..pos.0).all(|x| self[(x, pos.1)] < tree_pos)
            || (pos.0 + 1..self.width).all(|x| self[(x, pos.1)] < tree_pos)
            || (0..pos.1).all(|y| self[(pos.0, y)] < tree_pos)
            || (pos.1 + 1..self.height).all(|y| self[(pos.0, y)] < tree_pos)
    }

    fn visible_trees(&self) -> usize {
        (0..self.width)
            .map(|x| {
                (0..self.height)
                    .filter(|&y| self.is_visible((x, y)))
                    .count()
            })
            .sum()
    }

    fn score_at(&self, pos: TreePosition) -> usize {
        let tree_pos = self[pos];
        let left = pos.0
            - (0..pos.0)
                .rev()
                .find(|&x| self[(x, pos.1)] >= tree_pos)
                .unwrap_or(0);
        let right = (pos.0 + 1..self.width)
            .find(|&x| self[(x, pos.1)] >= tree_pos)
            .unwrap_or(self.width - 1)
            - pos.0;
        let top = pos.1
            - (0..pos.1)
                .rev()
                .find(|&y| self[(pos.0, y)] >= tree_pos)
                .unwrap_or(0);
        let bottom = (pos.1 + 1..self.height)
            .find(|&y| self[(pos.0, y)] >= tree_pos)
            .unwrap_or(self.height - 1)
            - pos.1;

        left * right * top * bottom
    }

    fn scenic_score(&self) -> usize {
        (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| self.score_at((x, y)))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

fn part_one(input: &str) -> usize {
    let grid = TreeGrid::from(input);
    grid.visible_trees()
}

fn part_two(input: &str) -> usize {
    let grid = TreeGrid::from(input);
    grid.scenic_score()
}

pub fn run() {
    let input = include_str!("../input/day8/input.txt");

    println!("DAY 8:");
    timing(|| part_one(input), 1);
    timing(|| part_two(input), 2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part_one() {
        assert_eq!(21, part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(8, part_two(INPUT));
    }
}
