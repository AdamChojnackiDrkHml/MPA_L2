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
        let res_uni = single_test(n, rep, graph::Graph::create_full_graph_uniform);

        write_res_to_file(&mut file_uni, &res_uni, n);

        let res_norm = single_test(n, rep, graph::Graph::create_full_graph_normal);

        write_res_to_file(&mut file_norm, &res_norm, n);
    }

}

fn sum_edges(mst: &Vec<Edge>) -> f64 {
    mst.iter().map(|elem| elem.weight).sum()
}

fn single_test(n: usize, rep: usize, generator: impl Fn(usize) -> graph::Graph) -> Vec<(f64, f64, f64)> {
    let mut res: Vec<(f64, f64, f64)> = vec![];

    for _ in 0..rep {
        let g = generator(n);
        let (_, t_k) = get_exec_time(kruskal::kruskal, &g);
        let (edges, t_p) = get_exec_time(prim::prim, &g);

        res.push((t_k, t_p, sum_edges(&edges)));
    }

    return res;
}

fn get_exec_time(body: impl Fn(&graph::Graph) -> Vec<Edge>, args: &graph::Graph) -> (Vec<Edge>, f64) {
    let start_time = Instant::now();
    let result = body(args);
    let total_time =
        start_time.elapsed().as_secs() as f64 + start_time.elapsed().subsec_nanos() as f64 / 1e9;
    (result, total_time)
}

fn write_res_to_file(file: &mut File, res: &Vec<(f64, f64, f64)>, n: usize) {
    let str_vec: Vec<String> = res.into_iter().map(|(k, p, s)| format!("{};{};{};{}", n, k, p, s)).collect();

    file.write_all((str_vec.join("\n") + "\n").as_bytes()).unwrap();
}