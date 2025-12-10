use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, Variable, default_solver, variable,
};
use std::{str::FromStr, vec};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Button {
    light_indices: Vec<usize>,
}

impl FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let light_indices = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(",")
            .map(str::trim)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Ok(Button { light_indices })
    }
}

#[derive(Debug)]
struct Light {
    on: bool,
}

struct LightPuzzle {
    lights: Vec<Light>,
    buttons: Vec<Button>,
    joltages: Vec<u64>,
}

impl FromStr for LightPuzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let lights = parts[0]
            .trim_matches(|c| c == '[' || c == ']')
            .chars()
            .map(|c| match c {
                '.' => Light { on: false },
                '#' => Light { on: true },
                _ => panic!("Invalid light"),
            })
            .collect();
        let buttons = parts[1..parts.len() - 1]
            .iter()
            .map(|s| Button::from_str(s).unwrap())
            .collect();
        let joltages = parts[parts.len() - 1]
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(str::trim)
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Ok(LightPuzzle {
            lights,
            buttons,
            joltages,
        })
    }
}

impl LightPuzzle {
    fn find_optimal_solution_lights(&self) -> u64 {
        let num_lights = self.lights.len();
        let num_buttons = self.buttons.len();
        let mut matrix = vec![vec![0u8; num_buttons + 1]; num_lights];

        for (i, b) in self.buttons.iter().enumerate() {
            for &l in b.light_indices.iter() {
                matrix[l][i] = 1;
            }
        }

        for (i, l) in self.lights.iter().enumerate() {
            if l.on {
                matrix[i][num_buttons] = 1;
            }
        }

        let mut pivot_cols = vec![];
        let mut current_row = 0;
        for col in 0..num_buttons {
            if let Some(pivot_row) = (current_row..num_lights).find(|&r| matrix[r][col] == 1) {
                matrix.swap(current_row, pivot_row);

                for row in 0..num_lights {
                    if row != current_row && matrix[row][col] == 1 {
                        let (r1, r2) = if row < current_row {
                            let (left, right) = matrix.split_at_mut(current_row);
                            (&mut left[row], &mut right[0])
                        } else {
                            let (left, right) = matrix.split_at_mut(row);
                            (&mut right[0], &mut left[current_row])
                        };
                        r1.iter_mut().zip(r2.iter()).for_each(|(c1, &c2)| *c1 ^= c2);
                    }
                }
                pivot_cols.push((current_row, col));
                current_row += 1;
            }
        }

        for row in &matrix[current_row..] {
            assert_ne!(row[num_buttons], 1);
        }

        let mut is_pivot = vec![false; num_buttons];

        for (_, col) in pivot_cols.clone() {
            is_pivot[col] = true;
        }

        let free_vars: Vec<_> = (0..num_buttons).filter(|&i| !is_pivot[i]).collect();

        let mut min_presses = u64::MAX;

        for mask in 0..(1u64 << free_vars.len()) {
            let mut solution = vec![0u8; num_buttons];

            for (bit, &var) in free_vars.iter().enumerate() {
                solution[var] = ((mask >> bit) & 1) as u8;
            }

            for &(row, col) in pivot_cols.iter().rev() {
                let mut sum = matrix[row][num_buttons];
                for c in (col + 1)..num_buttons {
                    sum ^= matrix[row][c] & solution[c];
                }
                solution[col] = sum;
            }

            let presses = solution.iter().map(|&x| x as u64).sum();
            min_presses = min_presses.min(presses);
        }

        min_presses
    }

    fn find_optimal_solution_joltages(&self) -> u64 {
        let mut vars = ProblemVariables::new();

        let buttons: Vec<Variable> = (0..self.buttons.len())
            .map(|_| vars.add(variable().integer().min(0).max(1000)))
            .collect();

        let mut problem = vars
            .minimise(buttons.iter().sum::<Expression>())
            .using(default_solver);

        for (light_idx, &target_joltage) in self.joltages.iter().enumerate() {
            // Find which buttons affect this light
            let constraint: Expression = self
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, btn)| btn.light_indices.contains(&light_idx))
                .map(|(btn_idx, _)| buttons[btn_idx])
                .sum();

            problem = problem.with(constraint.eq(target_joltage as f64));
        }

        // Solve
        let solution = problem.solve().unwrap();

        // Sum up the button presses
        buttons
            .iter()
            .map(|&var| solution.value(var).round() as u64)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzles: Vec<LightPuzzle> = input
        .trim()
        .lines()
        .map(|l| l.parse::<LightPuzzle>().unwrap())
        .collect();
    let p: Vec<u64> = puzzles
        .iter()
        .map(|p| p.find_optimal_solution_lights())
        .collect();
    Some(p.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles: Vec<LightPuzzle> = input
        .trim()
        .lines()
        .map(|l| l.parse::<LightPuzzle>().unwrap())
        .collect();
    let p: Vec<u64> = puzzles
        .iter()
        .map(|p| p.find_optimal_solution_joltages())
        .collect();
    Some(p.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
