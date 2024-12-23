use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

fn main() {
    let now = Instant::now();
    p1();
    let elapsed = now.elapsed();
    println!("p1: {:.2?}", elapsed);

    let now = Instant::now();
    p2();
    let elapsed = now.elapsed();
    println!("p2: {:.2?}", elapsed);
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

fn p1() {
    let links = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| {
            let s = line.split("-").collect::<Vec<_>>();
            (s[0].to_string(), s[1].to_string())
        })
        .collect::<Vec<_>>();

    let connection_graph = get_connections(links);
    let tris = find_triangles(connection_graph);

    // sum every string that contains a node that starts with a "t"
    let sum = tris
        .iter()
        .filter(|tri| tri.iter().any(|node| node.starts_with("t")))
        .collect::<Vec<_>>()
        .len();

    println!("p1: {}", sum);
}

// finds max cliques in a graph
fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    graph: &Graph,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r); // report R as a maximal clique
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

fn p2() {
    let links = fs::read_to_string("./src/input.txt")
        .expect("Should have been able to read the file")
        .trim_end()
        .lines()
        .map(|line| {
            let s = line.split("-").collect::<Vec<_>>();
            (s[0].to_string(), s[1].to_string())
        })
        .collect::<Vec<_>>();

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
    max_clique.sort(); // sort alphabetically

    println!("p2: {:?}", max_clique.join(","));
}
