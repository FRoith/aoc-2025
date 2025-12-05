use std::str::FromStr;

advent_of_code::solution!(5);

struct Ingredient {
    id: u64,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: s.parse().unwrap(),
        })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct FreshnessRange {
    start: u64,
    end: u64,
}

impl FromStr for FreshnessRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        Ok(Self { start, end })
    }
}

#[derive(Debug)]
struct FreshnessRanges {
    items: Vec<FreshnessRange>,
}

impl FreshnessRanges {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, item: FreshnessRange) {
        self.items.push(item);
    }

    fn dedup(&mut self) {
        let (items, deduped) = self.items.iter_mut().fold(
            (Vec::new(), false),
            |(mut v, mut deduped): (Vec<FreshnessRange>, bool), i| {
                for j in v.iter_mut() {
                    if j.contains(i.start) {
                        if !j.contains(i.end) {
                            deduped = true;
                            j.end = i.end;
                        } else {
                            break;
                        }
                    } else if j.contains(i.end) {
                        if !j.contains(i.start) {
                            deduped = true;
                            j.start = i.start;
                        } else {
                            break;
                        }
                    } else if i.contains(j.start) {
                        if !i.contains(j.end) {
                            deduped = true;
                            j.start = i.start;
                        } else {
                            j.start = i.start;
                            j.end = i.end;
                            break;
                        }
                    } else if i.contains(j.end) {
                        if !i.contains(j.start) {
                            deduped = true;
                            j.end = i.end;
                        } else {
                            j.start = i.start;
                            j.end = i.end;
                            break;
                        }
                    }
                }
                if v.iter()
                    .any(|fr| fr.contains(i.start) && fr.contains(i.end))
                {
                    (v, deduped)
                } else {
                    v.push(i.clone());
                    (v, deduped)
                }
            },
        );
        self.items = items;
        if deduped {
            self.dedup();
        }
    }
}

impl FreshnessRange {
    fn contains(&self, id: u64) -> bool {
        self.start <= id && id <= self.end
    }
}

struct FreshnessTracker {
    freshness_ranges: FreshnessRanges,
    ingredients: Vec<Ingredient>,
}

impl FromStr for FreshnessTracker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut freshness_ranges = FreshnessRanges::new();
        let mut ingredients = Vec::new();

        let mut ss = s.splitn(2, "\n\n");

        for line in ss.next().unwrap().trim().lines() {
            freshness_ranges.add(line.parse().unwrap());
        }

        for line in ss.next().unwrap().trim().lines() {
            let ingredient = line.parse().unwrap();
            ingredients.push(ingredient);
        }

        Ok(Self {
            freshness_ranges,
            ingredients,
        })
    }
}

impl FreshnessTracker {
    fn get_num_fresh(&self) -> u64 {
        self.ingredients
            .iter()
            .filter(|i| {
                self.freshness_ranges
                    .items
                    .iter()
                    .any(|fr| fr.contains(i.id))
            })
            .count() as u64
    }

    fn get_total_num_fresh(&mut self) -> u64 {
        self.freshness_ranges.dedup();
        self.freshness_ranges
            .items
            .iter()
            .fold(0, |acc, fr| acc + fr.end - fr.start + 1)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.parse::<FreshnessTracker>().unwrap().get_num_fresh())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .parse::<FreshnessTracker>()
            .unwrap()
            .get_total_num_fresh(),
    )
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
        assert_eq!(result, Some(14));
    }
}
