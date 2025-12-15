use std::collections::HashSet;
use std::str::FromStr;

use std::collections::HashMap;

advent_of_code::solution!(12, 1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    mask: [[bool; 3]; 3],
}

impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = [[false; 3]; 3];
        for (i, line) in s.trim().lines().skip(1).enumerate() {
            if i > 2 {
                return Err(format!("Too many lines: {}", i));
            }
            for (j, c) in line.trim().chars().enumerate() {
                if j > 2 {
                    return Err(format!("Too many columns: {}", j));
                }
                mask[i][j] = c == '#';
            }
        }
        Ok(Shape { mask })
    }
}

impl Shape {
    fn rot90(&self) -> Self {
        let mut mask = [[false; 3]; 3];
        for (i, row) in mask.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.mask[3 - j - 1][i];
            }
        }
        Shape { mask }
    }

    fn mirror_h(&self) -> Self {
        let mut mask = [[false; 3]; 3];
        for (i, row) in mask.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.mask[i][3 - j - 1];
            }
        }
        Shape { mask }
    }

    fn mirror_v(&self) -> Self {
        let mut mask = [[false; 3]; 3];
        for (i, row) in mask.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.mask[3 - i - 1][j];
            }
        }
        Shape { mask }
    }

    fn get_all(&self) -> HashSet<Shape> {
        let mut set = HashSet::new();
        let rots: [Self; 4] = [
            self.clone(),
            self.rot90(),
            self.rot90().rot90(),
            self.rot90().rot90().rot90(),
        ];
        for rot in rots {
            set.insert(rot.mirror_h());
            set.insert(rot.mirror_v());
            set.insert(rot);
        }
        set
    }

    fn combine(&self, other: &Self) -> usize {
        for i in 0..3 {
            for j in 0..3 {
                let mut ok = true;
                for i2 in 0..(3 - i) {
                    for j2 in 0..(3 - j) {
                        if self.mask[i2][j2] && other.mask[i + i2][j + j2] {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        break;
                    }
                }
                if ok {
                    return (3 + i) * (3 + j);
                }
            }
        }
        6 * 3
    }
}

struct Board {
    counts: [usize; 6],
    shape: (usize, usize),
}

impl FromStr for Board {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(": ");
        let mut iit = it.next().unwrap().trim().split("x");
        let (width, height) = (
            usize::from_str(iit.next().unwrap()).unwrap(),
            usize::from_str(iit.next().unwrap()).unwrap(),
        );
        let iit = it.next().unwrap().split_whitespace();
        let mut counts = [0; 6];
        for (i, c) in iit.enumerate() {
            if i > 5 {
                return Err(format!("Too many shapes: {}", i));
            }
            counts[i] = usize::from_str(c).unwrap();
        }
        Ok(Board {
            counts,
            shape: (width, height),
        })
    }
}

impl Board {
    fn size(&self) -> usize {
        self.shape.0 * self.shape.1
    }

    fn estimate(&self, shape_pairs: &[((usize, usize), usize)]) -> usize {
        let mut counts = self.counts;
        let mut acc = 0;
        for ((i, j), v) in shape_pairs {
            if counts[*i] > 0 && counts[*j] > 0 {
                if *i == *j {
                    acc += v * (counts[*i] / 2);
                    counts[*i] %= 2;
                } else {
                    let n = counts[*i].min(counts[*j]);
                    acc += v * n;
                    counts[*i] -= n;
                    counts[*j] -= n;
                }
            }
        }
        for c in counts {
            acc += c * 9;
        }
        acc
    }
}

fn build_shape_pairs(shapes: &[Shape]) -> Vec<((usize, usize), usize)> {
    let mut hm: HashMap<(usize, usize), usize> = HashMap::new();

    for (i, s1) in shapes.iter().enumerate() {
        for (j, s2) in shapes.iter().enumerate() {
            let s1s = s1.get_all();
            let s2s = s2.get_all();
            for s1s in s1s.iter() {
                for s2s in s2s.iter() {
                    let c = s1s.combine(s2s);
                    let k = if i <= j { (i, j) } else { (j, i) };
                    let n = if hm.contains_key(&k) {
                        hm[&k]
                    } else {
                        usize::MAX
                    };
                    if c < n {
                        hm.insert(k, c);
                    }
                }
            }
        }
    }
    let mut v: Vec<((usize, usize), usize)> = hm.iter().map(|(&k, &v)| (k, v)).collect();
    v.sort_by_key(|k| k.1);
    v
}

pub fn part_one(input: &str) -> Option<u64> {
    let it = input.trim().split("\n\n");
    let shapes = it
        .take(6)
        .map(Shape::from_str)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let boards = input
        .trim()
        .split("\n\n")
        .last()
        .unwrap()
        .trim()
        .lines()
        .map(Board::from_str)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let mut acc = 0;
    let shape_pairs = build_shape_pairs(&shapes);
    for board in boards {
        if board.size() > board.estimate(&shape_pairs) {
            acc += 1;
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
