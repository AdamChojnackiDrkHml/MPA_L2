use std::vec;

use crate::graph::{Graph, Edge};


struct Subset {
    parent: usize,
    rank: usize,
}

fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    match subsets[i].parent == i {
        true => subsets[i].parent,
        false => find(subsets, subsets[i].parent),
    }  
}

fn union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let x_root: usize = find(subsets, x);
    let y_root: usize = find(subsets, y);

    if subsets[x_root].rank < subsets[y_root].rank {
        subsets[x_root].parent = y_root;
    }  else if subsets[x_root].rank > subsets[y_root].rank {
        subsets[y_root].parent = x_root;
    } else {
        subsets[y_root].parent = x_root;
        subsets[x_root].rank += 1;
    }
}

pub fn kruskal(g: &Graph) -> Vec<Edge> {
    let mut edges: Vec<Edge> = g.get_edges();
    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    let mut subsets: Vec<Subset> = (0..g.size)
        .into_iter()
        .map(|i| -> Subset {
            Subset{parent: i, rank: 0}
        })
        .collect();

    let mut mst: Vec<Edge> = vec![]; 
    let mut mst_size: usize = 0;

    for edge in edges.iter() {
        if find(&mut subsets, edge.src) != find(&mut subsets, edge.dest) {
            mst.push(edge.clone());
            union(&mut subsets, edge.src, edge.dest);
            mst_size += 1;
        }
        if mst_size == g.size + 1 {break;}
    }

    mst
}

