use std::io::{BufRead};
use std::error::Error;
use std::collections::HashSet;

pub struct Node {
    pub neighbours: HashSet<u16>, //vecinii unui nod
}

pub struct Graph{
    pub adj: Vec<Node>, // vector de noduri
    pub zones: Vec<Vec<(u8, u8)>>, // mat[0] contine zonele care au activarea 0
    // prima var din fiecare field = id ul nodului, a doua este resursa
}

#[derive(Clone)]
#[derive(Default)]
pub struct Convertor {
    pub conv_type: bool, //0 = 2 la 1, 1 = 3 la 1
    pub resource: u8,
    pub nodes: Vec<u16>
}

impl Graph {
    
    pub fn new(reader:&mut impl BufRead) -> Self {  
        let mut grid: Vec<Node> = Vec::new();
        let mut zones: Vec<Vec<(u8, u8)>> = Vec::new();

        for _i in 0..54 {
            grid.push(Node {neighbours: HashSet::new()});
        }

        for _i in 0..=12 {
            zones.push(Vec::new());
        }

        for _j in 0..19 {
            
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
                // zones[j].nodes.push(line_values[i]);
                zones[line_values[1] as usize].push((line_values[i] as u8, line_values[0] as u8));
                // grid[line_values[i] as usize].zones.push(j as u16); //adaugam si id ul zonei
            }
    
            // zones[j].component = line_values[0];
            // zones[j].light = line_values[1];

            line.clear();
        }
        
        
        Self {
            adj: grid,
            zones: zones,
        }
    }

    pub fn print(self) {

        let graph = &self;
        
        for i in 0..12 {
            println!("{i}");
            for j in &graph.zones[i] {
                print!("{a} {b}, ", a = j.0, b = j.1)
            }
            println!("");
            
        }
    
        println!("");
        
        for i in 0..54 {
            print!("{i} Neighbours:");
            for j in &graph.adj[i].neighbours {
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

pub fn read_graph(reader: &mut impl BufRead) -> Result<(Graph, u8, u16, Vec<Convertor>, Vec<u8>), Box<dyn Error>>  
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

    reader.read_line(&mut line)?;

    let days_vec: Vec<u8> = line.split_whitespace().map(|day| day.parse::<u8>().unwrap()).collect();
    
    Ok((graph, plays, days, convertors, days_vec))
}