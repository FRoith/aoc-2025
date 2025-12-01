use std::str::FromStr;

advent_of_code::solution!(1);

enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(String::from("Invalid direction")),
        }
    }
}

struct Command {
    direction: Direction,
    distance: usize,
}

impl Command {
    fn move_(&self, position: usize) -> usize {
        match self.direction {
            Direction::Left => (position + 100 - self.distance % 100) % 100,
            Direction::Right => (position + self.distance % 100) % 100,
        }
    }

    fn move_count(&self, position: usize, count: usize) -> (usize, usize) {
        // also counts the number of times 0 is passed
        match self.direction {
            Direction::Left => (
                (position + 100 - self.distance % 100) % 100,
                ((self.distance) / 100) + (position <= (self.distance % 100)) as usize + count
                    - (position == 0) as usize,
            ),
            Direction::Right => (
                (position + self.distance % 100) % 100,
                ((position + self.distance) / 100) + count,
            ),
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s[0..1].parse::<Direction>().unwrap();
        let distance = s[1..].parse::<usize>().unwrap();
        Ok(Self {
            direction,
            distance,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let commands = input
        .trim()
        .split("\n")
        .map(|line| line.parse::<Command>().unwrap());
    // apply commands to position, remembering all previous positions
    let positions = commands.fold(vec![50], |mut positions, command| {
        let p = command.move_(*positions.last().unwrap_or(&0));
        positions.push(p);
        positions
    });
    Some(positions.iter().filter(|p| **p == 0).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let commands = input
        .trim()
        .split("\n")
        .map(|line| line.parse::<Command>().unwrap());
    // apply commands to position, remembering all previous positions
    let (_final_pos, final_count) = commands
        .fold((50, 0), |(position, count): (usize, usize), command| {
            command.move_count(position, count)
        });
    Some(final_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
