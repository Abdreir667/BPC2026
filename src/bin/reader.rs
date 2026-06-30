use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

struct Zone {
    component: u16, //ce tip de componenta este
    light: u16,
    nodes: Vec<u16> //nodurile care fac parte dintr-o zona
}

struct Node {
    zones: Vec<u16>, //zonele din care face parte un nod
    neighbours: Vec<u16> //vecinii unui nod
}

struct Graph{
    adj: Vec<Node>, // vector de noduri
    zones: Vec<Zone>
}

impl Graph {
    
    pub fn new(reader:&mut BufReader<File>) -> Self {  
        let mut grid: Vec<Node> = Vec::new();
        let mut zones: Vec<Zone> = Vec::new();

        for _i in 0..54 {
            grid.push(Node {zones: vec![], neighbours: vec![]});
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
            grid[line_values[2] as usize].neighbours.push(line_values[3]);
            grid[line_values[2] as usize].neighbours.push(line_values[7]);
            grid[line_values[7] as usize].neighbours.push(line_values[2]);
            grid[line_values[7] as usize].neighbours.push(line_values[6]);
            
            for i in 2..=7 {
    
                if i >=3 && i <=6 {
                    grid[line_values[i] as usize].neighbours.push(line_values[i + 1]);
                    grid[line_values[i] as usize].neighbours.push(line_values[i - 1]);
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
}

fn main() -> Result<(), Box<dyn Error>>  
{
    let path: &str = "/home/radulescuandrei/Facultate/Anul1/BPC2026/public_blueprints/south_01.in";
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let plays: u8 = line.trim().parse::<u8>()?; 
    line.clear();

    reader.read_line(&mut line)?;

    let days: u16 = line.trim().parse::<u16>()?;

    println!("{plays} {days}");

    let graph = Graph::new(&mut reader);

    Ok(())
}