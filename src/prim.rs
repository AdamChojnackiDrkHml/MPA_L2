use std::{collections::{HashSet, BinaryHeap}, cmp::Reverse};

use crate::graph::{Graph, Edge};

pub fn prim(graph: &Graph) -> Vec<Edge> {
    let mut mst: Vec<Edge> = vec![];
    let mut visited = HashSet::new();
    let start_vertex = 0;

    let mut min_heap = BinaryHeap::new();
    visited.insert(start_vertex);

    for &(neighbor, weight) in &graph.nodes[start_vertex].adj_vec {
        min_heap.push(Reverse(Edge {
            src: start_vertex,
            dest: neighbor,
            weight 
        }));
    }

    while visited.len() < graph.size {
        let smallest_edge = min_heap.pop().unwrap();

        if visited.contains(&smallest_edge.0.dest) {
            continue;
        }

        visited.insert(smallest_edge.0.dest);
        mst.push(smallest_edge.clone().0);

        for &(neighbor, weight) in &graph.nodes[smallest_edge.0.dest].adj_vec {
            if !visited.contains(&neighbor) {
                min_heap.push(Reverse(Edge {
                    src: smallest_edge.0.dest,
                    dest: neighbor,
                    weight 
                }));
            }
        }
    }

    mst
}
