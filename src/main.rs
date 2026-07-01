mod reader;

use crate::reader::Graph;
use crate::reader::read_graph as read_graph;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    let graph: Graph;
    let plays; let days;

    let path = "/home/radulescuandrei/Facultate/Anul1/BPC2026/public_blueprints/south_01.in";

    (graph, plays, days) = read_graph(path)?;

    graph.print();


    Ok(())
}