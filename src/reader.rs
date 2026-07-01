use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashSet;

struct Zone {
    component: u16, //ce tip de componenta este
    light: u16,
    nodes: Vec<u16> //nodurile care fac parte dintr-o zona
}

struct Node {
    zones: Vec<u16>, //zonele din care face parte un nod
    neighbours: HashSet<u16> //vecinii unui nod
}

pub struct Graph{
    adj: Vec<Node>, // vector de noduri
    zones: Vec<Zone>
}



impl Graph {
    
    pub fn new(reader:&mut impl BufRead) -> Self {  
        let mut grid: Vec<Node> = Vec::new();
        let mut zones: Vec<Zone> = Vec::new();

        for _i in 0..54 {
            grid.push(Node {zones: vec![], neighbours: HashSet::new()});
        }

        for _i in 0..19 {
            zones.push(Zone {component: 0, light: 0,  nodes: vec![]});
        }


        for j in 0..19 {
            
            let mut line: String = String::new();
            reader.read_line(&mut line).unwrap();
            
            let line_values: Vec<u16> = line.split_whitespace().map(|piece| piece.parse::<u16>().unwrap()).collect();
            //ia linia, o imparte in elemente despartite de whitespace-uri (strtok), si pt fiecare chestie o parseaza la u16
    
            //conectam primul nod cu ultimul si ultimul cu primul, nu ar fi mers in for
            grid[line_values[2] as usize].neighbours.insert(line_values[3]);
            grid[line_values[2] as usize].neighbours.insert(line_values[7]);
            grid[line_values[7] as usize].neighbours.insert(line_values[2]);
            grid[line_values[7] as usize].neighbours.insert(line_values[6]);
            
            for i in 2..=7 {
    
                if i >=3 && i <=6 {
                    grid[line_values[i] as usize].neighbours.insert(line_values[i + 1]);
                    grid[line_values[i] as usize].neighbours.insert(line_values[i - 1]);
                }
                zones[j].nodes.push(line_values[i]);
                grid[line_values[i] as usize].zones.push(j as u16); //adaugam si id ul zonei
            }
    
            zones[j].component = line_values[0];
            zones[j].light = line_values[1];

            line.clear();
        }
        
        
        Self {
            adj: grid,
            zones: zones,
        }
    }

    pub fn print(self) {

        let graph = &self;
        
        for i in 0..19 {
            println!("{param1} {param2}", param1=graph.zones[i].component, param2 = graph.zones[i].light);
            for j in &graph.zones[i as usize].nodes {
                print!("{j} ");
            }
            println!("");
        }
    
        println!("");
        
        for i in 0..54 {
            print!("{i} Neighbours:");
            for j in &graph.adj[i].neighbours {
                print!("{j} ");
            }
            print!("Zones: ");
            for j in &graph.adj[i].zones {
                print!("{j} ");
            }
            println!("");
        }
        
    }
}

pub fn read_graph(reader: &mut impl BufRead) -> Result<(Graph, u8, u16), Box<dyn Error>>  
{

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let plays: u8 = line.trim().parse::<u8>()?; 
    line.clear();

    reader.read_line(&mut line)?;

    let days: u16 = line.trim().parse::<u16>()?;

    println!("{plays} {days}");

    let graph = Graph::new(reader);

    
    Ok((graph, plays, days))
}