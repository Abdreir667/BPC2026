mod reader;

use crate::reader::Graph;
use crate::reader::read_graph as read_graph;
use std::error::Error;
use std::io::*;
use std::fs::File;

fn main(){

    let graph: Graph;
    let plays; let days;

    let use_file: bool = false;
    
    if use_file {
        let path = "/home/radulescuandrei/Facultate/Anul1/BPC2026/public_blueprints/south_01.in";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        (graph, plays, days) = read_graph(&mut reader).unwrap();
    } else {
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();

        (graph, plays, days) = read_graph(&mut reader).unwrap();
    }

    graph.print();

}