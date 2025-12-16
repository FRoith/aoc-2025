use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn get_all_repeating(&self) -> Vec<usize> {
        (self.start.ilog10() + 1..=self.end.ilog10() + 1)
            .filter(|n| n.is_multiple_of(2))
            .flat_map(|n| {
                (10_usize.pow((n / 2) - 1)..10_usize.pow(n / 2)).filter_map(move |m| {
                    let v = m * (10_usize.pow(n / 2) + 1);
                    if self.start <= v && v <= self.end {
                        Some(v)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn get_smore_repeating(&self) -> HashSet<usize> {
        (self.start.ilog10() + 1..=self.end.ilog10() + 1)
            .flat_map(|nn| {
                (2..=nn)
                    .filter(move |&n| nn.is_multiple_of(n))
                    .flat_map(move |n| {
                        (10_usize.pow((nn / n) - 1)..10_usize.pow(nn / n)).filter_map(move |m| {
                            let mut v = m;
                            for _ in 1..n {
                                v *= 10_usize.pow(nn / n);
                                v += m;
                            }
                            if self.start <= v && v <= self.end {
                                Some(v)
                            } else {
                                None
                            }
                        })
                    })
            })
            .collect()
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        let start = parts.next().unwrap().parse::<usize>().unwrap();
        let end = parts.next().unwrap().parse::<usize>().unwrap();
        Ok(Self::new(start, end))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(",")
            .map(|s| s.parse::<Range>().unwrap())
            .fold(vec![], |mut v, r| {
                v.extend(r.get_all_repeating());
                v
            })
            .iter()
            .sum::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(",")
            .map(|s| s.parse::<Range>().unwrap())
            .fold(vec![], |mut v, r| {
                v.extend(r.get_smore_repeating());
                v
            })
            .iter()
            .sum::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(18700015741));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(20077272987));
    }
}
