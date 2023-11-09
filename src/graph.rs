use std::{fmt, cmp::Ordering};
use float_next_after::NextAfter;
use rand::{distributions::Uniform, Rng};
use rand_distr::{Normal, Distribution};

pub struct Graph {
    pub size: usize,
    pub nodes: Vec<Node>,
    pub matrix: Vec<Vec<f64>>
}

pub struct Node {
    pub label: usize,
    pub adj_vec: Vec<(usize, f64)>
}

#[derive(Clone, Copy, fmt::Debug, PartialEq)]
pub struct Edge {
    pub src: usize,
    pub dest: usize, 
    pub weight: f64,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: Vec<String> = (&self.matrix).into_iter().map(|v| v.iter().fold(String::new(), |acc, &arg| acc + "\t| |\t"  + &format!("{:.10}", arg))).collect();
        write!(f, "{}", str.join("\n"))
    }
}

impl Graph {

    pub fn create_full_graph_uniform(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let range: Uniform<f64> = Uniform::new(0.0.next_after(std::f64::INFINITY), 1.0);
        let mut new_graph = Graph { size: n, nodes: vec![], matrix: vec![vec![0.0; n]; n] };
        

        for i in 0..n {
            let adj: Vec<f64> = (i+1..n).map(|_| rng.sample(&range)).collect();

            new_graph.matrix[i][i] = 0.0;
            for j in i+1..n {
                new_graph.matrix[i][j] = adj[j - i - 1];
                new_graph.matrix[j][i] = adj[j - i - 1];
            }

            let weights: Vec<f64> = new_graph.matrix[i].clone();
            let mut node_adj: Vec<(usize, f64)> = weights.into_iter().enumerate().collect();
            node_adj.remove(i);
            let node: Node = Node { label: i, adj_vec: node_adj };
            new_graph.nodes.push(node);
        }

        new_graph
    }

    pub fn create_full_graph_normal(n: usize) -> Self {
        let normal = Normal::new(0.5, 0.15).unwrap();
        let mut new_graph = Graph { size: n, nodes: vec![], matrix: vec![vec![0.0; n]; n] };

        for i in 0..n {
            let adj: Vec<f64> = (i+1..n).map(|_| -> f64 {
                let mut rand: f64 = normal.sample(&mut rand::thread_rng());

                rand = if rand <= 0.0 {0.0.next_after(std::f64::INFINITY)} else {rand};
                
                rand = if rand >= 1.0 {1.0.next_after(std::f64::NEG_INFINITY)} else {rand};

                rand

            }).collect();

            new_graph.matrix[i][i] = 0.0;
            for j in i+1..n {
                new_graph.matrix[i][j] = adj[j - i - 1];
                new_graph.matrix[j][i] = adj[j - i - 1];
            }
            
            let weights: Vec<f64> = new_graph.matrix[i].clone();
            let mut node_adj: Vec<(usize, f64)> = weights.into_iter().enumerate().collect();
            node_adj.remove(i);
            let node: Node = Node { label: i, adj_vec: node_adj };
            new_graph.nodes.push(node);
        }

        new_graph
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = vec![];

        for (i, row) in self.matrix.iter().enumerate() {
            for j in i+1..row.len() {
                edges.push(Edge { src: i, dest: j, weight: row[j]})
            }
        }
    
        edges
    }

}

