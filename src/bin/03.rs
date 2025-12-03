use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(3);

#[derive(Eq)]
struct Battery {
    joltage: u64,
    position: usize,
}

impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.joltage == other.joltage
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.joltage.partial_cmp(&other.joltage)
    }
}

impl Ord for Battery {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.joltage.cmp(&other.joltage)
    }
}

impl Display for Battery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.joltage)
    }
}

struct BatteryLine {
    batteries: Vec<Battery>,
}

impl Display for BatteryLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.batteries
                .iter()
                .map(|b| b.to_string())
                .collect::<String>()
        )
    }
}

impl FromStr for BatteryLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut batteries = Vec::new();
        for j in s.trim().chars() {
            if j.is_ascii_digit() {
                batteries.push(Battery {
                    joltage: j.to_digit(10).unwrap() as u64,
                    position: batteries.len(),
                });
            }
        }
        Ok(BatteryLine { batteries })
    }
}

impl BatteryLine {
    fn get_max_joltage(&self, n: usize) -> u64 {
        //print!("{}: ", self);
        let mut val = 0;
        let mut i = 0;
        let end = self.batteries.len() - n;
        for nn in 0..n {
            let b = self.batteries[i..=end + nn].iter().rev().max().unwrap();
            val *= 10;
            val += b.joltage;
            i = b.position + 1;
        }
        //println!("{}", val);
        val
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| line.parse::<BatteryLine>().unwrap().get_max_joltage(2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| line.parse::<BatteryLine>().unwrap().get_max_joltage(12))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
