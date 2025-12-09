use std::{collections::BTreeMap, str::FromStr};

advent_of_code::solution!(9);

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Point { x, y })
    }
}

impl Point {
    fn get_area(&self, other: &Point) -> u64 {
        ((self.x - other.x).unsigned_abs() + 1) * ((self.y - other.y).unsigned_abs() + 1)
    }

    fn get_lines(&self, other: &Point) -> Vec<Line> {
        let mut lines = Vec::new();
        if self.x == other.x {
            lines.push(Line::Vertical {
                x: self.x,
                ymin: self.y,
                ymax: other.y,
            });
        } else if self.y == other.y {
            lines.push(Line::Horizontal {
                y: self.y,
                xmin: self.x,
                xmax: other.x,
            });
        } else {
            let xmin = self.x.min(other.x) + 1;
            let xmax = self.x.max(other.x) - 1;
            let ymin = self.y.min(other.y) + 1;
            let ymax = self.y.max(other.y) - 1;
            lines.push(Line::Horizontal {
                y: ymin,
                xmin,
                xmax,
            });
            lines.push(Line::Horizontal {
                y: ymax,
                xmin,
                xmax,
            });
            lines.push(Line::Vertical {
                x: xmin,
                ymin,
                ymax,
            });
            lines.push(Line::Vertical {
                x: xmax,
                ymin,
                ymax,
            });
        }
        lines
    }
}

enum Line {
    Horizontal { y: i64, xmin: i64, xmax: i64 },
    Vertical { x: i64, ymin: i64, ymax: i64 },
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        let xmin = p1.x.min(p2.x);
        let xmax = p1.x.max(p2.x);
        let ymin = p1.y.min(p2.y);
        let ymax = p1.y.max(p2.y);
        if xmin == xmax {
            Line::Vertical {
                x: xmin,
                ymin,
                ymax,
            }
        } else if ymin == ymax {
            Line::Horizontal {
                y: ymin,
                xmin,
                xmax,
            }
        } else {
            panic!("Invalid line")
        }
    }
}

struct Map {
    points: Vec<Point>,
    horizontal: BTreeMap<i64, Vec<(i64, i64)>>,
    vertical: BTreeMap<i64, Vec<(i64, i64)>>,
}

impl Map {
    fn add_line(&mut self, line: Line) {
        match line {
            Line::Horizontal { y, xmin, xmax } => {
                self.horizontal.entry(y).or_default().push((xmin, xmax));
            }
            Line::Vertical { x, ymin, ymax } => {
                self.vertical.entry(x).or_default().push((ymin, ymax));
            }
        }
    }

    fn num_intersections(&self, line: &Line) -> usize {
        match line {
            Line::Horizontal { y, xmin, xmax } => {
                if xmin > xmax {
                    return 0;
                }
                self.vertical
                    .range(xmin..=xmax)
                    .flat_map(|(_, ys)| ys.iter().filter(|(y1, y2)| y1 <= y && y2 >= y))
                    .count()
            }
            Line::Vertical { x, ymin, ymax } => {
                if ymin > ymax {
                    return 0;
                }
                self.horizontal
                    .range(ymin..=ymax)
                    .flat_map(|(_, xs)| xs.iter().filter(|(x1, x2)| x1 <= x && x2 >= x))
                    .count()
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s.lines().map(str::parse).map(Result::unwrap).collect();
        let mut s = Map {
            points,
            horizontal: BTreeMap::new(),
            vertical: BTreeMap::new(),
        };
        let mut prev_point = s.points.last().unwrap();
        for point in &s.points.clone() {
            let line = Line::new(prev_point.clone(), point.clone());
            s.add_line(line);
            prev_point = point;
        }
        Ok(s)
    }
}

impl Map {
    fn find_largest_area(&self) -> u64 {
        let mut largest_area = 0;
        for (i, p1) in self.points.iter().enumerate() {
            for p2 in self.points[i + 1..].iter() {
                let area = p1.get_area(p2);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
        largest_area
    }

    fn find_largest_valid_area(&self) -> u64 {
        let mut largest_area = 0;
        for (i, p1) in self.points.iter().enumerate() {
            for p2 in self.points[i + 1..].iter() {
                let area = p1.get_area(p2);
                let lines = p1.get_lines(p2);
                if area > largest_area && lines.iter().all(|line| self.num_intersections(line) == 0)
                {
                    largest_area = area;
                }
            }
        }
        largest_area
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.trim().parse::<Map>().unwrap();
    Some(map.find_largest_area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.trim().parse::<Map>().unwrap();
    Some(map.find_largest_valid_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
