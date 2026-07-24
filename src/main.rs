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
    let mut start_state = DynamicState::new();

    let start_time = Instant::now();

    while !start_state.is_game_over(&board) {
        let mut solver = MCTSearch::new(&start_state, &board);

        let chosen_move  = solver.search(&start_state, &board, 1000);

        let is_setup = !matches!(start_state.phase, GamePhase::NormalState);
        start_state.apply_move(chosen_move, &board, is_setup);
    }

    let fin = start_time.elapsed();

    println!("{}", start_state.get_points());

    
    println!("Total time: {:?}", fin);
    println!("Time in milliseconds: {} ms", fin.as_millis());
    println!("Time in microseconds: {} µs", fin.as_micros());
}