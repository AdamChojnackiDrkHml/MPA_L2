use std::{time::Instant, fs::File, io::Write};


use graph::Edge;

pub mod graph;
pub mod kruskal;
pub mod prim;

fn main() {
    let min_n = 200;
    let max_n = 5000;
    let step = 40;
    let rep = 100;

    let mut file_uni = File::create("data/resultTestUni.csv").unwrap();
    let mut file_norm = File::create("data/resultTestNorm.csv").unwrap();
    
    for n in (min_n..max_n).step_by(step) {
        let (t_k, t_p, s) = single_test(n, rep, graph::Graph::create_full_graph_uniform);

        file_uni.write_all((format!("{};{};{:+e}", t_k, t_p, s) + "\n").as_bytes()).unwrap();

        let (t_k, t_p, s) = single_test(n, rep, graph::Graph::create_full_graph_normal);

        file_norm.write_all((format!("{};{};{:+e}", t_k, t_p, s) + "\n").as_bytes()).unwrap();
    }

}

fn sum_edges(mst: &Vec<Edge>) -> f64 {
    mst.iter().map(|elem| elem.weight).sum()
}

fn single_test(n: usize, rep: usize, generator: impl Fn(usize) -> graph::Graph) -> (f64, f64, f64) {
    let mut sum_t_k = 0.0;
    let mut sum_t_p = 0.0;
    let mut sum_size = 0.0;

    for _ in 0..rep {
        let g = generator(n);
        let (_, t_k) = get_exec_time(kruskal::kruskal, &g);
        let (edges, t_p) = get_exec_time(prim::prim, &g);

        sum_t_k += t_k;
        sum_t_p += t_p;
        sum_size += sum_edges(&edges) as f64;
    }

    let rep_f = rep as f64;

    (sum_t_k / rep_f, sum_t_p / rep_f, sum_size / rep_f)
}

fn get_exec_time(body: impl Fn(&graph::Graph) -> Vec<Edge>, args: &graph::Graph) -> (Vec<Edge>, f64) {
    let start_time = Instant::now();
    let result = body(args);
    let total_time =
        start_time.elapsed().as_secs() as f64 + start_time.elapsed().subsec_nanos() as f64 / 1e9;
    (result, total_time)
}