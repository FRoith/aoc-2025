use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Eq, PartialEq, Clone)]
enum RollType {
    Roll,
    NoRoll,
    ClearedRoll,
}

impl From<char> for RollType {
    fn from(c: char) -> Self {
        match c {
            '@' => RollType::Roll,
            '.' => RollType::NoRoll,
            'x' => RollType::ClearedRoll,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Clone)]
struct RollMap {
    map: HashMap<(isize, isize), RollType>,
}

impl RollMap {
    fn new(input: &str) -> Self {
        let mut map: HashMap<(isize, isize), RollType> = HashMap::new();
        let lines: Vec<&str> = input.trim().lines().map(|l| l.trim()).collect();
        let h = lines.len();
        let w = lines[0].len();
        for y in 0..h {
            let cs = lines[y].chars().collect::<Vec<char>>();
            for x in 0..w {
                map.insert((x as isize, y as isize), cs[x].into());
            }
        }
        RollMap { map }
    }

    fn get_num_neighbors(&self, x: isize, y: isize) -> usize {
        let mut num = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                if let Some(nt) = self.map.get(&(nx, ny))
                    && *nt == RollType::Roll
                {
                    num += 1;
                }
            }
        }
        num
    }

    fn clear_rolls(&mut self, max_neighbors: usize) {
        let orig_map = self.clone();
        self.map.iter_mut().for_each(|((x, y), v)| {
            if *v == RollType::Roll && orig_map.get_num_neighbors(*x, *y) < max_neighbors {
                *v = RollType::ClearedRoll;
            }
        });
    }

    fn count_cleared(&self) -> u64 {
        self.map
            .iter()
            .filter(|(_, v)| **v == RollType::ClearedRoll)
            .count() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut roll_map = RollMap::new(input);
    roll_map.clear_rolls(4);
    Some(roll_map.count_cleared())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut roll_map = RollMap::new(input);
    roll_map.clear_rolls(4);
    let mut nc = roll_map.count_cleared();
    loop {
        roll_map.clear_rolls(4);
        let nc_new = roll_map.count_cleared();
        if nc_new == nc {
            break;
        }
        nc = nc_new;
    }
    Some(nc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
