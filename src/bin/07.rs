use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    vec,
};

advent_of_code::solution!(7);

struct BeamMap {
    size: (usize, usize),
    beams: HashSet<(usize, usize)>,
    split_points: HashSet<(usize, usize)>,
}

impl FromStr for BeamMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut beams = HashSet::new();
        let mut split_points = HashSet::new();
        let mut xmax = 0;
        let mut ymax = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    beams.insert((y, x));
                }
                if c == '^' {
                    split_points.insert((y, x));
                }
                xmax = x.max(xmax);
                ymax = y.max(ymax);
            }
        }
        Ok(BeamMap {
            size: (ymax + 1, xmax + 1),
            beams,
            split_points,
        })
    }
}

impl BeamMap {
    fn count_splits(&mut self) -> u64 {
        let mut splits = 0;
        let mut beams = self.beams.clone();
        for y in 1..self.size.0 {
            beams = beams
                .iter()
                .flat_map(|s| {
                    if self.split_points.contains(&(y, s.1)) {
                        splits += 1;
                        vec![(y, s.1 - 1), (y, s.1 + 1)]
                    } else {
                        vec![(y, s.1)]
                    }
                })
                .collect();
        }
        splits
    }

    fn count_possilbilities(&mut self) -> u64 {
        let mut beams: HashMap<(usize, usize), usize> =
            self.beams.iter().map(|b| (*b, 1)).collect();
        for y in 1..self.size.0 {
            beams = beams
                .iter()
                .flat_map(|(s, n)| {
                    if self.split_points.contains(&(y, s.1)) {
                        vec![((y, s.1 - 1), n), ((y, s.1 + 1), n)]
                    } else {
                        vec![((y, s.1), n)]
                    }
                })
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    if acc.contains_key(&k) {
                        acc.insert(k, acc.get(&k).unwrap() + v);
                    } else {
                        acc.insert(k, *v);
                    }
                    acc
                });
        }
        beams.values().sum::<usize>() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .trim()
        .parse::<BeamMap>()
        .ok()
        .map(|mut bm| bm.count_splits())
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .trim()
        .parse::<BeamMap>()
        .ok()
        .map(|mut bm| bm.count_possilbilities())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
