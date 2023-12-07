use std::{time::Instant, fs::File, io::Write};
use rand::{seq::IteratorRandom, distributions::Uniform, Rng, thread_rng};


use graph::{Edge, Graph};

pub mod graph;
pub mod kruskal;
pub mod prim;
pub mod propagator;

fn main() {
    let min_n = 100;
    let max_n = 10100;
    let step = 500;
    let rep = 30;
    let rep_root = 30;

    let mut file_uni = File::create("data/dataL3/resultTestUniFin2.csv").unwrap();
    let mut file_norm = File::create("data/dataL3/resultTestNormFin2.csv").unwrap();
    
    // for n in (min_n..max_n).step_by(step) {
    //     let res_uni = single_test(n, rep, graph::Graph::create_full_graph_uniform);

    //     write_res_to_file(&mut file_uni, &res_uni, n);

    //     let res_norm = single_test(n, rep, graph::Graph::create_full_graph_normal);

    //     write_res_to_file(&mut file_norm, &res_norm, n);
    // }

    file_uni.write_all("n;rounds;time\n".as_bytes()).unwrap();
    file_norm.write_all("n;rounds;time\n".as_bytes()).unwrap();

    for n in (min_n..max_n).step_by(step) {
        let res_uni = single_test_propagator(n, rep, rep_root, graph::Graph::create_full_graph_uniform);

        write_res_to_file_propagator(&mut file_uni, &res_uni, n);

        let res_norm = single_test_propagator(n, rep, rep_root, graph::Graph::create_full_graph_normal);

        write_res_to_file_propagator(&mut file_norm, &res_norm, n);

        println!("Done {}", n)
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

fn write_res_to_file_propagator(file: &mut File, res: &Vec<(usize, f64)>, n: usize) {
    let str_vec: Vec<String> = res.into_iter().map(|(r, t)| format!("{};{};{}", n, r, t)).collect();

    file.write_all((str_vec.join("\n") + "\n").as_bytes()).unwrap();
}

fn test_propagator(mst: &Vec<Edge>) -> usize {
    let rng = rand::thread_rng().gen_range(0..mst.len() - 1);
    propagator::propagate(mst, rng)
}

fn average(numbers: &Vec<usize>) -> f32 {
    numbers.iter().sum::<usize>() as f32 / numbers.len() as f32
}

fn single_test_propagator(n: usize, rep: usize, rep_root: usize, generator: impl Fn(usize) -> graph::Graph) -> Vec<(usize, f64)> {
    let mut res: Vec<(usize, f64)> = vec![];

    for _ in 0..rep {
        let g = generator(n);
        let mst = prim::prim(&g);
        let mut rng = thread_rng();
        assert!(rep_root <= &mst.len() - 1);
        let sample = (0 as usize..&mst.len()-1).into_iter().choose_multiple(&mut rng, rep_root);
        for root in sample.into_iter() {
            let start_time = Instant::now();
            let rounds = propagator::propagate(&mst, root);
            let total_time = start_time.elapsed().as_secs() as f64 + start_time.elapsed().subsec_nanos() as f64 / 1e9;
            res.push((rounds, total_time));
        }
    }

    return res;
}