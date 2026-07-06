use std::io::{BufRead};
use std::error::Error;
use std::collections::HashSet;

pub struct Zone {
    pub component: u16, //ce tip de componenta este
    pub light: u16,
    pub nodes: Vec<u16> //nodurile care fac parte dintr-o zona
}

pub struct Node {
    pub zones: Vec<u16>, //zonele din care face parte un nod
    pub neighbours: HashSet<u16>, //vecinii unui nod
}

pub struct Graph{
    pub adj: Vec<Node>, // vector de noduri
    pub zones: Vec<Zone>
}

#[derive(Clone)]
#[derive(Default)]
pub struct Convertor {
    conv_type: bool, //0 = 2 la 1, 1 = 3 la 1
    resource: u8,
    nodes: Vec<u16>
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

fn parse_int(str: &String) -> u32 {
    let mut temp: u32 = str.chars().find(|a| a.is_digit(10)).and_then(|a| a.to_digit(10)).unwrap();
    if temp > 10 {
        temp /= 10;
    }
    temp
}

pub fn read_graph(reader: &mut impl BufRead) -> Result<(Graph, u8, u16, Vec<Convertor>), Box<dyn Error>>  
{

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let plays: u8 = line.trim().parse::<u8>()?; 
    line.clear();

    reader.read_line(&mut line)?;

    let days: u16 = line.trim().parse::<u16>()?;

    let graph = Graph::new(reader);

    line.clear();

    let mut convertors: Vec<Convertor> = vec![Default::default(); 6];

    for i in 0..6 {
        reader.read_line(&mut line)?;

        let conv_type: Vec<String> = line.split_whitespace().map(|value| value.to_string()).collect();

        if conv_type[0].len() == 2 {
            convertors[i].conv_type = false;
        } else {
            convertors[i].conv_type = true;
        }

        convertors[i].resource = parse_int(&conv_type[0]) as u8;

        let conv_slice = &conv_type[2..];
        let nodes: Vec<u16> = conv_slice.iter().map(|value| value.parse::<u16>().unwrap()).collect();
        
        convertors[i].nodes = nodes;

        // print!("{param1} {param2}\n", param1 = convertors[i].resource, param2 = convertors[i].conv_type);
        // 
        // for j in &convertors[i].nodes {
        //     print!("{j} ");
        // }
        // println!("");

        line.clear();
        
    }
    
    Ok((graph, plays, days, convertors))
}