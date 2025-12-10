use std::collections::{BTreeSet, HashMap};

use graphalgs::{
    connect::scc::tarjan_scc,
    mst::kruskal,
    petgraph::{data::Element, graph::UnGraph},
};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Edge {
    coord1: (i64, i64, i64),
    coord2: (i64, i64, i64),
    weight: i64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub fn get_graph(
    coords: &[(i64, i64, i64)],
    limit: Option<usize>,
) -> UnGraph<&(i64, i64, i64), i64> {
    let mut graph: UnGraph<&(i64, i64, i64), i64> = UnGraph::new_undirected();
    let nodes = coords
        .iter()
        .map(|c| (c, graph.add_node(c)))
        .collect::<HashMap<_, _>>();
    let mut edges = BTreeSet::new();

    for pair in coords.iter().combinations(2) {
        let [&c1, &c2] = pair[0..2] else {
            unreachable!()
        };
        let weight = (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) + (c1.2 - c2.2).pow(2);
        edges.insert(Edge {
            coord1: c1,
            coord2: c2,
            weight,
        });
    }
    if let Some(n) = limit {
        edges = edges.into_iter().take(n).collect();
    }

    edges.iter().for_each(
        |Edge {
             coord1,
             coord2,
             weight,
         }| {
            graph.add_edge(nodes[&coord1], nodes[&coord2], *weight);
        },
    );

    graph
}

pub fn part1(input: Vec<String>) -> usize {
    let coords: Vec<(i64, i64, i64)> = input
        .iter()
        .map(|l| {
            l.split(',')
                .flat_map(|n| n.parse::<i64>())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    let graph = get_graph(&coords, Some(1000));

    tarjan_scc(&graph)
        .iter()
        .map(|g| g.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part2(input: Vec<String>) -> i64 {
    let coords: Vec<(i64, i64, i64)> = input
        .iter()
        .map(|l| {
            l.split(',')
                .flat_map(|n| n.parse::<i64>())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    let graph = get_graph(&coords, None);

    let kruskal = kruskal(&graph).collect_vec();
    let Element::Edge { source, target, .. } = kruskal[kruskal.len() - 1] else {
        unreachable!("Edges last")
    };
    let (Element::Node { weight: node1 }, Element::Node { weight: node2 }) =
        (&kruskal[source], &kruskal[target])
    else {
        unreachable!("Nodes first")
    };

    node1.0 * node2.0
}
