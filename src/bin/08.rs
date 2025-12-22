use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for JunctionBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(str::trim);
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Ok(JunctionBox { x, y, z })
    }
}

impl JunctionBox {
    fn sq_distance(&self, other: &JunctionBox) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as u64
    }
}

struct Connections {
    junctions: Vec<JunctionBox>,
    graph: Vec<Vec<usize>>,
    last_connection: Option<(usize, usize)>,
}

impl Connections {
    fn new(junctions: Vec<JunctionBox>, n_: Option<usize>) -> Self {
        let mut connections: Vec<((usize, usize), u64)> = Vec::new();
        let mut graph = Vec::new();
        let mut last_connection = None;
        let nnodes = junctions.len();

        for i in 0..nnodes {
            for j in i + 1..nnodes {
                connections.push(((i, j), junctions[i].sq_distance(&junctions[j])));
            }
            graph.push(Vec::new());
            graph[i].push(i);
        }
        let n = if let Some(n__) = n_ {
            n__
        } else {
            connections.len()
        };
        connections.sort_by(|a, b| a.1.cmp(&b.1));
        for ((a_, b_), _) in connections[0..n].iter() {
            let mut aa = graph
                .extract_if(0..graph.len(), |hs| hs.contains(a_))
                .next()
                .unwrap();
            if let Some(bb) = graph
                .extract_if(0..graph.len(), |hs| hs.contains(b_))
                .next()
            {
                aa.extend(bb);
            }
            graph.push(aa);
            if graph.len() == 1 {
                last_connection = Some((*a_, *b_));
                break;
            }
        }

        Self {
            junctions,
            graph,
            last_connection,
        }
    }

    fn get_num_circuits(&mut self) -> u64 {
        self.graph.sort_by_key(|g| std::cmp::Reverse(g.len()));
        self.graph[0].len() as u64 * self.graph[1].len() as u64 * self.graph[2].len() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions: Vec<JunctionBox> = input
        .trim()
        .lines()
        .map(|line| line.parse::<JunctionBox>().unwrap())
        .collect();
    let n = if junctions.len() < 1000 { 10 } else { 1000 };
    let mut connections = Connections::new(junctions, Some(n));

    Some(connections.get_num_circuits())
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions: Vec<JunctionBox> = input
        .trim()
        .lines()
        .map(|line| line.parse::<JunctionBox>().unwrap())
        .collect();
    let connections = Connections::new(junctions, None);
    let (j1, j2) = connections.last_connection.unwrap();

    Some(connections.junctions[j1].x as u64 * connections.junctions[j2].x as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
