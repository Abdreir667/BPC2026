use crate::reader::*;

use queues::*;

pub const ENERGY: usize = 0;
pub const WATER: usize = 1;
pub const DATA: usize = 2;
pub const RAM: usize = 3;
pub const GPU: usize = 4;

pub struct Board {
    pub graph: Graph,
    pub convertors: Vec<Convertor>,
    pub edge_id: [[u8; 54]; 54],
    pub turns: Vec<u8>,
}

#[derive(Clone, Copy)]
pub enum Move {
    BuildSettlement(u8),
    UpgradeSettlement(u8),
    BuildRoad(u8, u8),
    EndTurn,
    Trade(u8, u8, u8), //dam trade la x carduri din resursa 1 pt un card resursa 2
}

impl Move {
    pub fn weight(&self) -> u32 {
        match self {
            Move::UpgradeSettlement(_) => 100, 
            Move::BuildSettlement(_) => 95,
            Move::BuildRoad(_, _) => 30,
            Move::EndTurn => 10,
            Move::Trade(_, _, _) => 0,
        }
    }
}

#[derive(Clone)]
struct Node {
    visits: u32,
    current_score: f64,

    untried_actions: Vec<Move>,

    parent: Option<usize>,
    children: Vec<usize>
}

#[derive(Clone, Copy)]
pub struct DynamicState {
    turn_number: u8,
    points: u16,
    resources: [u8; 5],
    settlements: [u8; 54],
    pub phase: GamePhase,
    built_roads: u128,
}

#[derive(Clone, Copy)]
pub enum GamePhase {
    SetupCity1,
    SetupRoad1(u8),
    SetupCity2,
    SetupRoad2(u8),
    NormalState,
}

fn add_node(nodes: &mut u64, node: u8) {
    *nodes = *nodes | (1 << node);
}

fn used_node(nodes: u64, node: u8) -> bool {
    nodes & (1 << node) != 0
}

impl DynamicState {

    pub fn has_road(&self, u: u8, v: u8, board: &Board) -> bool {
        let edge_id = board.edge_id[u as usize][v as usize];
        if edge_id == 254 {
            false
        } else {
            self.built_roads & (1 << edge_id) != 0
        }
    }

    pub fn apply_move(&mut self, action: Move, board: &Board, start_turn: bool) {
        match action {
            Move::BuildRoad(u, v) => {
                let edge_id = board.edge_id[u as usize][v as usize];
                self.built_roads |= 1 << edge_id;
                if !start_turn {
                    self.resources[ENERGY] -= 1;
                    self.resources[WATER] -=1;
                }
                
                match self.phase {
                    GamePhase::SetupRoad1(_) => self.phase = GamePhase::SetupCity2,
                    GamePhase::SetupRoad2(_) => self.phase = GamePhase::NormalState,
                    _ => {}
                }
                
            }
            Move::BuildSettlement(x) => {
                self.settlements[x as usize] = 1;
                if !start_turn {
                    self.resources[ENERGY] -= 1;
                    self.resources[WATER] -=1;
                    self.resources[DATA] -= 1;
                    self.resources[RAM] -=1;
                }
                self.points += 1;

                match self.phase { 
                    GamePhase::SetupCity1 => self.phase = GamePhase::SetupRoad1(x),
                    GamePhase::SetupCity2 => {
                        self.phase = GamePhase::SetupRoad2(x);
                        self.resources[ENERGY] += 2;
                        self.resources[WATER] += 2;
                        self.resources[DATA] += 2;
                        self.resources[RAM] += 2;
                    }
                    _ => {}
                }
            }
            Move::UpgradeSettlement(x) => {
                let level = self.settlements[x as usize];
                self.settlements[x as usize] += 1;
                self.resources[RAM] -= level + 1;
                self.resources[GPU] -= level + 2;
                self.points += 1;
            }
            Move::Trade(x, y, z) => {
                self.resources[y as usize] -= x;
                self.resources[z as usize] += 1;
            }
            Move::EndTurn => {
                
                self.turn_number += 1;

                if (self.turn_number as usize) < board.turns.len() {
                    let next_roll = board.turns[self.turn_number as usize];
                    
                    for &(node_id, resource) in &board.graph.zones[next_roll as usize] {
                        let lvl = self.settlements[node_id as usize];
                        if lvl > 0 {
                            self.resources[resource as usize] += lvl;
                        }
                    }
                }
            }
        }
    }

    pub fn get_points(&self) -> u16 {
        self.points
    }
    
    pub fn is_game_over(&self, board: &Board) -> bool {
        (self.turn_number as usize) >= board.turns.len()
    }

    pub fn new() -> Self {
        Self {
            turn_number : 0,
            points: 0,
            resources: [0; 5],
            settlements: [0; 54],
            phase: GamePhase::SetupCity1,
            built_roads: 0,
        }
    }

    pub fn valid_settlement_spot(&self, board: &Board, start: u8) -> bool {
        if self.settlements[start as usize] > 0 {
            return false;
        }
        
        for &neigh in &board.graph.adj[start as usize].neighbours {
            if self.settlements[neigh as usize] > 0 {
                return false;
            }
        }

        return true;
    }
    
    fn find_settlement_nodes(&self, board: &Board, start: u8, used_nodes: &mut u64, moves: &mut Vec<Move>) {
        let mut q: Queue<u8> = queue![];
    
        add_node(used_nodes, start);
        
        for &i in &board.graph.adj[start as usize].neighbours {
            if self.has_road(start, i as u8, board) {
                q.add(i as u8).unwrap();
                add_node(used_nodes, i as u8);
                
                if self.valid_settlement_spot(board, i as u8) {
                    moves.push(Move::BuildSettlement(i as u8));
                }
            } else {
                moves.push(Move::BuildRoad(start, i as u8));
            }
        }
    
        while q.size() > 0 {
            let top: u8 = q.peek().unwrap();
            q.remove().unwrap();
    
            for &i in &board.graph.adj[top as usize].neighbours {
                if !used_node(*used_nodes, i as u8) && self.has_road(top, i as u8, board) {
                        q.add(i as u8).unwrap();
                        add_node(used_nodes, i as u8);

                        if self.valid_settlement_spot(board, i as u8) && (self.resources[DATA] >=1 && self.resources[RAM] >= 1 && self.resources[ENERGY] >= 1 && self.resources[WATER] >= 1) {
                            moves.push(Move::BuildSettlement(i as u8));
                        }
                } else if !used_node(*used_nodes, i as u8) && !self.has_road(top, i as u8, board) {
                    moves.push(Move::BuildRoad(top, i as u8));
                }
            }
        }
    }
    
    pub fn generate_legal_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = vec![];

        match self.phase {
            GamePhase::SetupCity1 | GamePhase::SetupCity2 => {
                for i in 0..54 {
                    if self.settlements[i] == 0 {

                        if self.valid_settlement_spot(board, i as u8) {
                            moves.push(Move::BuildSettlement(i as u8));
                        }
                    }
                }
            }

            GamePhase::SetupRoad1(start_idx) | GamePhase::SetupRoad2(start_idx) => {
                for i in &board.graph.adj[start_idx as usize].neighbours {
                    moves.push(Move::BuildRoad(start_idx, *i as u8));
                }
            }

            GamePhase::NormalState => {

                //place settlement and add roads that can be built
                if self.resources[ENERGY] >= 1 && self.resources[WATER] >= 1 || (self.resources[DATA] >=1 && self.resources[RAM] >= 1 && self.resources[ENERGY] >= 1 && self.resources[WATER] >= 1){
                    let mut used_nodes = 0;
                    for i in 0..54 {
                        if self.settlements[i] > 0 && !used_node(used_nodes, i as u8) {
                            self.find_settlement_nodes(board, i as u8, &mut used_nodes, &mut moves);
                        }
                    }
                }

                //upgrade settlement
                for i in 0..54 {
                    let lvl = self.settlements[i];
                    if lvl >= 1 {
                        if self.resources[RAM] >= lvl + 1 && self.resources[GPU] >= lvl + 2 {
                            moves.push(Move::UpgradeSettlement(i as u8));
                        }
                    }
                }
                
                for i in &board.convertors {
                    let needed_resource = if i.conv_type { 2 } else { 3 };
                    if self.resources[i.resource as usize] >= needed_resource {
                        for j in 0..5 {
                            if j != i.resource {
                                moves.push(Move::Trade(needed_resource, i.resource, j));
                            }
                        }
                    }
                }

                for i in 0..5 {
                    if self.resources[i] >= 4 {
                        for j in 0..5 {
                            if i != j { moves.push(Move::Trade(4, i as u8, j as u8));}
                        }
                    }
                }

                moves.push(Move::EndTurn);
                
            }

        }

        moves
    }
}

pub fn simulate_random_game(mut state: DynamicState, board: &Board) -> u16 {

    while !state.is_game_over(board) {
        let legal_moves = state.generate_legal_moves(board);

        if legal_moves.is_empty() {
            break;
        }

        let total_weight: u32 = legal_moves.iter().map(|m| m.weight()).sum();

        let mut rand_val = fastrand::u32(0..total_weight);
        let mut chosen_move = legal_moves[0];

        for current_move in &legal_moves {
            let move_weight = current_move.weight();
                    
            if rand_val < move_weight {
                chosen_move = *current_move;
                break;
            }
                    
            rand_val -= move_weight;
        }

        match state.phase {
            GamePhase::NormalState => {
                state.apply_move(chosen_move, board, false);
            }
            _ => {
                state.apply_move(chosen_move, board, true);
            }
        }

        // println!("{} {}", legal_moves.len(), state.turn_number);


        // match chosen_move {
        //     Move::EndTurn => { 
        //         println!("Ended turn {}", state.turn_number);
        //         println!("Resources: ENERGY: {} WATER : {} DATA : {} RAM : {} GPU : {} ", state.resources[0], state.resources[1], state.resources[2], state.resources[3], state.resources[4]);
        //     }
        //     Move::BuildRoad(x, y) => {println!("Built a road at {x} {y}");}
        //     Move::Trade(x, y, z) => {println!("Traded {x} cards from resource {y} for resource {z}");}
        //     Move::UpgradeSettlement(x) => {println!("Upgraded settlement {x}");}
        //     Move::BuildSettlement(x) => {println!("Built settlement {x}");}
        // }
    }

    let points = state.get_points();
    // println!("{points}");
    points
}

impl Node {

    pub fn new(state: DynamicState, board: &Board) -> Self {
        Self {
            visits: 0,
            current_score: 0.0,
            parent: None,
            untried_actions: state.generate_legal_moves(board),
            children: Vec::new(),
        }
    }

}

struct MCTSearch {
    nodes: Vec<Node>,
}

impl MCTSearch {
    pub fn new(root_node: Node) -> Self {
        Self {
            nodes: vec![root_node],
        }
    }

    fn add_child(&mut self, parent_idx: usize, mut child: Node) {
        let child_idx = self.nodes.len();
        child.parent = Some(parent_idx);
        self.nodes.push(child);
        self.nodes[parent_idx].children.push(child_idx);
    }

    // pub fn simulate(root_node: Node, )

}
