use std::fmt::Debug;
use std::{vec, usize};
use std::collections::HashMap;
use std::cmp::Reverse;

use crate::graph::Edge;


#[derive(Clone, Debug)]
struct PropagatorNode {
    max_depth: usize,
    subtrees: Vec<PropagatorNode>,
}

fn construct_adj_map(mst: &Vec<Edge>) -> HashMap<usize, Vec<usize>> {
    let mut adj_map: HashMap<usize, Vec<usize>> = HashMap::new(); 

    for edge in mst.iter() {
        adj_map.entry(edge.src)
            .and_modify(|e| e.push(edge.dest))
            .or_insert_with(|| vec![edge.dest]);
        adj_map.entry(edge.dest)
        .and_modify(|e| e.push(edge.src))
        .or_insert_with(|| vec![edge.src]);
    }

    adj_map
}

fn create_rooted_tree(tree_map: &HashMap<usize, Vec<usize>>, root: usize) -> PropagatorNode{
    create_rooted_node(tree_map, root, root)
}

fn create_rooted_node(tree_map: &HashMap<usize, Vec<usize>>, me: usize, parent: usize) -> PropagatorNode {
    let subtrees_indexes: Vec<usize> = tree_map.get(&me)
        .unwrap()
        .clone()
        .into_iter()
        .filter(|x| *x != parent)
        .collect();

    print!("Imgere");

    let subtrees = subtrees_indexes
        .into_iter()
        .map(|x| create_rooted_node(tree_map, x, me))
        .collect();

    println!("{:?}", subtrees);

    let mut roooted_node = PropagatorNode{
        max_depth: 0,
        subtrees,
    };  

    if roooted_node.subtrees.len() == 0 {
        roooted_node.max_depth = 1;
    } else {
        roooted_node.max_depth = 1 + roooted_node.subtrees
        .iter()
        .map(|node| node.max_depth)
        .max()
        .unwrap();

        roooted_node.subtrees.sort_by_key(|a| Reverse(a.max_depth));
    }

    roooted_node
}

pub fn propagate(mst: &Vec<Edge>, root: usize) -> usize {
    let rooted_tree = create_rooted_tree(&construct_adj_map(mst), root);
    println!("{:?}", rooted_tree);
    propagate_node(&rooted_tree)
}

fn propagate_node(p_node: &PropagatorNode) -> usize {
    let mut round = 1;
    let mut rounds: Vec<usize> = Vec::new();

    

    for s_tree in p_node.subtrees.iter() {
        let rounds_in_node = propagate_node(&s_tree); 
        rounds.push(rounds_in_node + round);
        round += 1;
    }

    print!("ADFSA");
    match rounds.len() {
        0 => 1,
        _ => *rounds.iter_mut().max().unwrap(), 
    }
}