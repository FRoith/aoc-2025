use std::{str::FromStr, vec};

use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;
use i_overlay::i_float::float::point::FloatPoint;
use i_overlay::i_shape::float::area::Area;
advent_of_code::solution!(9);

struct Map {
    polygon: Vec<Vec<FloatPoint<f64>>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut polygon = Vec::new();
        let mut points = Vec::new();
        for line in s.lines() {
            let mut line_iter = line.split(',');
            let point = FloatPoint {
                x: line_iter.next().unwrap().parse().unwrap(),
                y: line_iter.next().unwrap().parse().unwrap(),
            };
            points.push(point.clone());
        }
        polygon.push(points);
        Ok(Map { polygon })
    }
}

impl Map {
    fn find_largest_area(&self) -> u64 {
        let mut largest_area = 0;
        for p1 in &self.polygon[0] {
            for p2 in &self.polygon[0] {
                let xmin = p1.x.min(p2.x);
                let xmax = p1.x.max(p2.x);
                let ymin = p1.y.min(p2.y);
                let ymax = p1.y.max(p2.y);
                let rect = [vec![
                    FloatPoint {
                        x: xmin - 1.,
                        y: ymin - 1.,
                    },
                    FloatPoint {
                        x: xmax,
                        y: ymin - 1.,
                    },
                    FloatPoint { x: xmax, y: ymax },
                    FloatPoint {
                        x: xmin - 1.,
                        y: ymax,
                    },
                ]];
                let area = rect.area().abs() as u64;
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
        largest_area
    }

    fn find_largest_valid_area(&self) -> u64 {
        let mut largest_area = 0;
        for p1 in &self.polygon[0] {
            for p2 in &self.polygon[0] {
                let xmin = p1.x.min(p2.x);
                let xmax = p1.x.max(p2.x);
                let ymin = p1.y.min(p2.y);
                let ymax = p1.y.max(p2.y);
                let rect = [vec![
                    FloatPoint {
                        x: xmin - 1.,
                        y: ymin - 1.,
                    },
                    FloatPoint {
                        x: xmax,
                        y: ymin - 1.,
                    },
                    FloatPoint { x: xmax, y: ymax },
                    FloatPoint {
                        x: xmin - 1.,
                        y: ymax,
                    },
                ]];
                let inner_rect = [vec![
                    FloatPoint { x: xmin, y: ymin },
                    FloatPoint { x: xmax, y: ymin },
                    FloatPoint { x: xmax, y: ymax },
                    FloatPoint { x: xmin, y: ymax },
                ]];
                let area = rect.area().abs() as u64;
                let overlay_rule = OverlayRule::Intersect;
                let fill_rule = FillRule::Positive;
                if area > largest_area
                    && self
                        .polygon
                        .overlay(&inner_rect, overlay_rule, fill_rule)
                        .area()
                        == inner_rect.area()
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
