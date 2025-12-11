use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(11);

struct Node {
    name: String,
    connections: Vec<String>,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':').map(str::trim);
        let name = parts.next().unwrap().to_string();
        let connections = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Ok(Node { name, connections })
    }
}

struct NodeMap {
    map: HashMap<String, Vec<String>>,
}

impl NodeMap {
    fn new(nodes: impl Iterator<Item = Node>) -> Self {
        let mut map = HashMap::new();
        for node in nodes {
            map.insert(node.name, node.connections);
        }
        map.insert("out".to_string(), vec!["out".to_string()]);
        Self { map }
    }

    fn get_num_connections(&self) -> u64 {
        let mut positions = HashMap::<String, u64>::new();
        positions.insert("you".to_string(), 1);

        while positions.len() > 1 || !positions.contains_key("out") {
            positions = positions.iter().fold(HashMap::new(), |mut hm, (k, v)| {
                let kvec = self.map.get(k).unwrap();
                for k2 in kvec {
                    let v2 = hm.entry(k2.clone()).or_insert(0);
                    *v2 += v;
                }
                hm
            });
        }
        *positions.get("out").unwrap()
    }

    fn get_num_valid_connections(&self) -> u64 {
        let mut positions = HashMap::<String, (u64, u64, u64, u64)>::new();
        positions.insert("svr".to_string(), (1, 0, 0, 0));

        while positions.len() > 1 || !positions.contains_key("out") {
            positions = positions.iter().fold(HashMap::new(), |mut hm, (k, v)| {
                let kvec = self.map.get(k).unwrap();
                for k2 in kvec {
                    let v2 = hm.entry(k2.clone()).or_insert((0, 0, 0, 0));
                    if k2 == "fft" {
                        v2.1 += v.0;
                        v2.3 += v.2;
                    } else if k2 == "dac" {
                        v2.2 += v.0;
                        v2.3 += v.1;
                    } else {
                        v2.0 += v.0;
                        v2.1 += v.1;
                        v2.2 += v.2;
                        v2.3 += v.3;
                    }
                }
                hm
            });
        }
        positions.get("out").unwrap().3
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let node_map = NodeMap::new(input.trim().lines().map(|l| l.parse::<Node>().unwrap()));
    Some(node_map.get_num_connections())
}

pub fn part_two(input: &str) -> Option<u64> {
    let node_map = NodeMap::new(input.trim().lines().map(|l| l.parse::<Node>().unwrap()));
    Some(node_map.get_num_valid_connections())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
