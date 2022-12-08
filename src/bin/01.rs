use itertools;
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::Lines;

fn caloreis(lines: Lines) -> Vec<u32> {
    let calories = lines
        //split lines into groups by empty line
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, group)| {
            group
                .map(|el| el.parse::<u32>())
                .try_collect::<u32, Vec<u32>, ParseIntError>()
        })
        .filter(|group| group.is_ok())
        .map(|group| group.map(|group| group.iter().sum::<u32>()))
        .map(|group| group.unwrap())
        .collect::<Vec<_>>();
    calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let calories = caloreis(lines);
    calories.iter().max().map(|max| *max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut calories = caloreis(lines);
    calories.sort();
    Some(calories.iter().rev().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
