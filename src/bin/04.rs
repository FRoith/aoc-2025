use ndarray::{Array2, arr2, s};
use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Clone)]
struct RollMap {
    h: usize,
    w: usize,
    rolls: HashSet<(usize, usize)>,
    map: Array2<u8>,
}

impl RollMap {
    fn new(input: &str) -> Self {
        let mut rolls: HashSet<(usize, usize)> = HashSet::new();
        let lines: Vec<&str> = input.trim().lines().map(|l| l.trim()).collect();
        let h: usize = lines.len();
        let w: usize = lines[0].len();
        let mut map = Array2::from_elem((h, w), 0);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    rolls.insert((x, y));

                    let mut sl = map.slice_mut(s![
                        y.saturating_sub(1)..=(y + 1).min(h - 1),
                        x.saturating_sub(1)..=(x + 1).min(w - 1)
                    ]);
                    sl.iter_mut().for_each(|v| *v += 1);
                }
            }
        }
        RollMap { h, w, rolls, map }
    }

    fn clear_rolls(&mut self, max_neighbors: u8) -> usize {
        let l = self.rolls.len();
        let orig_map = self.map.clone();
        self.rolls = self
            .rolls
            .iter()
            .filter_map(|&(x, y)| {
                if *orig_map.get((y, x)).unwrap() > max_neighbors {
                    Some((x, y))
                } else {
                    let mut sl = self.map.slice_mut(s![
                        y.saturating_sub(1)..=(y + 1).min(self.h - 1),
                        x.saturating_sub(1)..=(x + 1).min(self.w - 1)
                    ]);
                    sl.iter_mut().for_each(|v| *v -= 1);
                    None
                }
            })
            .collect();
        l - self.rolls.len()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut roll_map = RollMap::new(input);
    Some(roll_map.clear_rolls(4) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut roll_map = RollMap::new(input);
    let mut nc = roll_map.clear_rolls(4);
    loop {
        let nc_new = roll_map.clear_rolls(4);
        if nc_new == 0 {
            break;
        }
        nc += nc_new;
    }
    Some(nc as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1505));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(9182));
    }
}
