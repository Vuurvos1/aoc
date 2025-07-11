use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl Solution for Day23 {
    type Part1 = usize;
    type Part2 = String;

    fn day(&self) -> u32 {
        23
    }

    fn solve_p1(&self, input: &str) -> Self::Part1 {
        let links = parse_input(input);
        let connection_graph = get_connections(links);
        let tris = find_triangles(connection_graph);

        tris.iter()
            .filter(|tri| tri.iter().any(|node| node.starts_with('t')))
            .count()
    }

    fn solve_p2(&self, input: &str) -> Self::Part2 {
        let links = parse_input(input);
        let connection_graph = get_connections(links);
        let mut cliques: Vec<HashSet<String>> = Vec::new();
        bron_kerbosch(
            HashSet::new(),
            connection_graph.keys().cloned().collect(),
            HashSet::new(),
            &connection_graph,
            &mut cliques,
        );

        let mut max_clique: Vec<String> = cliques
            .iter()
            .max_by_key(|clique| clique.len())
            .unwrap()
            .iter()
            .cloned()
            .collect();
        max_clique.sort();

        max_clique.join(",")
    }
}

type Graph = HashMap<String, HashSet<String>>;
type Triangle = [String; 3];

fn get_connections(links: Vec<(String, String)>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (a, b) in links {
        graph.entry(a.clone()).or_default().insert(b.clone());
        graph.entry(b).or_default().insert(a);
    }
    graph
}

fn find_triangles(graph: Graph) -> HashSet<Triangle> {
    let mut triangles: HashSet<Triangle> = HashSet::new();
    for (node, connections) in &graph {
        for (i, n1) in connections.iter().enumerate() {
            for n2 in connections.iter().skip(i + 1) {
                if graph.get(n1).unwrap().contains(n2) {
                    let mut tri: Triangle = [node.clone(), n1.clone(), n2.clone()];
                    tri.sort();
                    triangles.insert(tri);
                }
            }
        }
    }
    triangles
}

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    graph: &Graph,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    let mut p = p.clone();
    let mut x = x.clone();

    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let new_p: HashSet<_> = p.intersection(graph.get(&v).unwrap()).cloned().collect();
        let new_x: HashSet<_> = x.intersection(graph.get(&v).unwrap()).cloned().collect();

        bron_kerbosch(new_r, new_p, new_x, graph, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let s = line.split('-').collect::<Vec<_>>();
            (s[0].to_string(), s[1].to_string())
        })
        .collect::<Vec<_>>()
}
