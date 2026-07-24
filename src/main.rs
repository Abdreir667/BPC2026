mod reader;
mod mcts;

use crate::reader::*;
use crate::reader::read_graph as read_graph;
use std::io::*;
use std::fs::File;
use mcts::*;
use std::time::Instant;
use rayon::prelude::*;

fn main(){

    let graph: Graph;
    let plays; let days;

    let use_file: bool = true;
    let convertors: Vec<Convertor>;
    let days_vector: Vec<u8>;
    
    if use_file {
        let path = "/home/radulescuandrei/Facultate/Anul1/BPC2026/public_blueprints/south_01.in";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        (graph, plays, days, convertors, days_vector) = read_graph(&mut reader).unwrap();
    } else {
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();

        (graph, plays, days, convertors, days_vector) = read_graph(&mut reader).unwrap();
    }

    let mut edgeIds = [[255u8 ; 54]; 54];
    let mut current_idx = 0;

    for i in 0..54usize {
        for &j in &graph.adj[i].neighbours {
            if edgeIds[i][j as usize] == 255 {
                edgeIds[i][j as usize] = current_idx;
                edgeIds[j as usize][i] = current_idx;
                current_idx += 1;
            }
        }
    }

    // graph.print();
    
    let board: Board = Board {graph: graph, convertors: convertors, edge_id: edgeIds, turns: days_vector};
    let start_state = DynamicState::new();

    let start_time = Instant::now();

    let total_points: u64 = (0..1_000_000)
        .into_par_iter()
        .map(|_| {simulate_random_game(start_state, &board) as u64})
        .sum();

    let fin = start_time.elapsed();

    println!("Average: {}", total_points as f64 / 1_000_000.0);

    println!("Total time: {:?}", fin);
    println!("Time in milliseconds: {} ms", fin.as_millis());
    println!("Time in microseconds: {} µs", fin.as_micros());
}