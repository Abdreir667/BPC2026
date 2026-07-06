mod reader;
mod solvers;
mod mcts;

use crate::reader::*;
use crate::reader::read_graph as read_graph;
use std::io::*;
use std::fs::File;
use mcts::*;


fn main(){

    let graph: Graph;
    let plays; let days;

    let use_file: bool = true;
    let convertors: Vec<Convertor>;
    
    if use_file {
        let path = "/home/radulescuandrei/Facultate/Anul1/BPC2026/public_blueprints/south_01.in";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        (graph, plays, days, convertors) = read_graph(&mut reader).unwrap();
    } else {
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();

        (graph, plays, days, convertors) = read_graph(&mut reader).unwrap();
    }

    let board: Board = Board {settlements: vec![], graph: graph, convertors: convertors};
    

}