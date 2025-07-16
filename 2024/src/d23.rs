use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl Solution for Day23 {
    type Part1 = usize;
    type Part2 = String;

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
        let mut cliques: Vec<HashSet<&str>> = Vec::new();
        bron_kerbosch(
            HashSet::new(),
            connection_graph.keys().cloned().collect(),
            HashSet::new(),
            &connection_graph,
            &mut cliques,
        );

        let mut max_clique: Vec<&str> = cliques
            .iter()
            .max_by_key(|clique| clique.len())
            .unwrap()
            .iter()
            .copied()
            .collect();
        max_clique.sort();

        max_clique.join(",")
    }
}

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Triangle<'a> = [&'a str; 3];

fn get_connections<'a>(links: Vec<(&'a str, &'a str)>) -> Graph<'a> {
    let mut graph: Graph<'a> = HashMap::new();
    for (a, b) in links {
        graph.entry(a).or_default().insert(b);
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
                    let mut tri: Triangle = [node, n1, n2];
                    tri.sort();
                    triangles.insert(tri);
                }
            }
        }
    }
    triangles
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    p: HashSet<&'a str>,
    x: HashSet<&'a str>,
    graph: &Graph<'a>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    let mut p = p.clone();
    let mut x = x.clone();

    let vertices: Vec<_> = p.iter().cloned().collect();
    for v in vertices {
        let mut new_r = r.clone();
        new_r.insert(v);

        let new_p: HashSet<_> = p.intersection(graph.get(v).unwrap()).copied().collect();
        let new_x: HashSet<_> = x.intersection(graph.get(v).unwrap()).copied().collect();

        bron_kerbosch(new_r, new_p, new_x, graph, cliques);

        p.remove(&v);
        x.insert(v);
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim_end()
        .lines()
        .map(|line| {
            let s = line.split('-').collect::<Vec<_>>();
            (s[0], s[1])
        })
        .collect::<Vec<_>>()
}
