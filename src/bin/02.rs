use std::str::FromStr;

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
        let mut repeating = vec![];
        for i in self.start..=self.end {
            let s = i.to_string();
            let l = s.len();
            if l % 2 == 0 && s[..l / 2] == s[l / 2..] {
                repeating.push(i);
            }
        }
        repeating
    }

    fn get_smore_repeating(&self) -> Vec<usize> {
        let mut repeating = vec![];
        for i in self.start..=self.end {
            let s = i.to_string();
            let l = s.len();
            // find all strings which only consist of repeating patterns
            for j in 1..=l / 2 {
                if l % j == 0 && (j..=(l - j)).step_by(j).all(|n| s[0..j] == s[n..n + j]) {
                    repeating.push(i);
                    break;
                }
            }
        }
        repeating
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
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
