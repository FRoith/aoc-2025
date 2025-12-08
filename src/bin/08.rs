use petgraph::{
    algo::{connected_components, has_path_connecting, scc::kosaraju_scc},
    graph::UnGraph,
};
use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Clone, Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
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
        ((self.x as i64 - other.x as i64).pow(2)
            + (self.y as i64 - other.y as i64).pow(2)
            + (self.z as i64 - other.z as i64).pow(2)) as u64
    }
}

struct Connections {
    graph: UnGraph<u32, ()>,
    last_connection: Option<(JunctionBox, JunctionBox)>,
}

impl Connections {
    fn new(junctions: Vec<JunctionBox>, n_: Option<usize>) -> Self {
        let mut connections = Vec::new();
        let mut graph = UnGraph::<u32, ()>::new_undirected();
        let mut last_connection = None;

        for i in 0..junctions.len() {
            for j in i + 1..junctions.len() {
                connections.push((
                    (i as u32, j as u32),
                    junctions[i].sq_distance(&junctions[j]),
                ));
            }
            graph.add_node(1);
        }
        let n = if let Some(n__) = n_ {
            n__
        } else {
            connections.len()
        };
        connections.sort_by(|a, b| a.1.cmp(&b.1));
        for ((a_, b_), _) in connections[0..n].iter() {
            let a = (*a_).into();
            let b = (*b_).into();
            if !has_path_connecting(&graph, b, a, None) {
                graph.add_edge(a, b, ());
            }
            if connected_components(&graph) == 1 {
                last_connection =
                    Some((junctions[a.index()].clone(), junctions[b.index()].clone()));
                break;
            }
        }

        Self {
            graph,
            last_connection,
        }
    }

    fn get_num_circuits(&self) -> u64 {
        let mut sgs = kosaraju_scc(&self.graph);
        sgs.sort_by_key(|g| std::cmp::Reverse(g.len()));
        sgs[0].len() as u64 * sgs[1].len() as u64 * sgs[2].len() as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junctions: Vec<JunctionBox> = input
        .trim()
        .lines()
        .map(|line| line.parse::<JunctionBox>().unwrap())
        .collect();
    let n = if junctions.len() < 1000 { 10 } else { 1000 };
    let connections = Connections::new(junctions, Some(n));

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

    Some(j1.x * j2.x)
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
