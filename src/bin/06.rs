advent_of_code::solution!(6);

#[derive(Debug)]
enum MathOperation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: MathOperation,
}

impl MathProblem {
    fn new(numbers: Vec<u64>, operation: &str) -> MathProblem {
        let operation = match operation {
            "+" => MathOperation::Add,
            "*" => MathOperation::Multiply,
            _ => panic!("Invalid operation"),
        };
        MathProblem { numbers, operation }
    }

    fn solve(&self) -> u64 {
        match self.operation {
            MathOperation::Add => self.numbers.iter().sum::<u64>(),
            MathOperation::Multiply => self.numbers.iter().product::<u64>(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stuff: Vec<Vec<&str>> = input
        .lines()
        .map(|line| {
            let elements: Vec<&str> = line.split_whitespace().collect();
            elements
        })
        .collect();
    let mut problems: Vec<MathProblem> = Vec::new();
    for i in 0..stuff[0].len() {
        let numbers = (0..stuff.len() - 1)
            .map(|j| stuff[j][i].parse::<u64>().unwrap())
            .collect();
        problems.push(MathProblem::new(numbers, stuff[stuff.len() - 1][i]));
    }
    Some(problems.iter().map(|problem| problem.solve()).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    //split index is derived from one before the + or * in the last line
    let stuff: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            let elements = line.chars();
            elements.collect::<Vec<char>>()
        })
        .collect();

    let mut problems: Vec<MathProblem> = Vec::new();
    let mut prev_i = 0;
    for i in 1..stuff[0].len() {
        if stuff[stuff.len() - 1][i] != ' ' {
            let numbers = (prev_i..i)
                .filter_map(|ii| {
                    let s = (0..stuff.len() - 1)
                        .fold("".to_string(), |acc, jj| acc + &stuff[jj][ii].to_string());
                    s.trim().parse::<u64>().ok()
                })
                .collect();
            problems.push(MathProblem::new(
                numbers,
                stuff[stuff.len() - 1][prev_i].to_string().as_str(),
            ));
            prev_i = i;
        }
    }
    let numbers = (prev_i..stuff[0].len())
        .filter_map(|ii| {
            let s = (0..stuff.len() - 1)
                .fold("".to_string(), |acc, jj| acc + &stuff[jj][ii].to_string());
            s.trim().parse::<u64>().ok()
        })
        .collect();
    problems.push(MathProblem::new(
        numbers,
        stuff[stuff.len() - 1][prev_i].to_string().as_str(),
    ));
    Some(problems.iter().map(|problem| problem.solve()).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
