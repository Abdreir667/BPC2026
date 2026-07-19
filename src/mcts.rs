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
}

#[derive(Clone, Copy)]
pub enum Move {
    BuildSettlement(u8),
    UpgradeSettlement(u8),
    BuildRoad(u8, u8),
    EndTurn,
    Trade(u8, u8, u8), //dam trade la x carduri din resursa 1 pt un card resursa 2
}

#[derive(Clone)]
struct Node {
    visits: u32,
    current_score: f64,

    untriedActions: Vec<Move>,

    parent: Option<usize>,
    children: Vec<usize>
}

struct DynamicState {
    turn_number: u8,
    points: u8,
    resources: [u8; 5],
    settlements: [u8; 54],
    roads: [[u8; 2]; 54],
    pub phase: GamePhase,
    built_roads: u128,
}

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
    nodes & (1 << node) == 1
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

    pub fn apply_move(&mut self, action: Move, board: &Board) {
        match action {
            Move::BuildRoad(u, v) => {
                let edge_id = board.edge_id[u as usize][v as usize];
                self.built_roads |= 1 << edge_id;
            }
            Move::BuildSettlement(x) => {

            }
            Move::UpgradeSettlement(x) => {

            }
            Move::Trade(x, y, z) => {

            }
            Move::EndTurn => {

            }
        }
    }

    pub fn new(self) -> Self {
        Self {
            turn_number : 0,
            points: 0,
            resources: [0; 5],
            settlements: [0; 54],
            roads: [[0; 2]; 54],
            phase: GamePhase::SetupCity1,
            built_roads: 0,
        }
    }
    
    fn find_settlement_nodes(&self, board: &Board, start: u8, used_nodes: &mut u64, moves: &mut Vec<Move>) {
        let mut q: Queue<u8> = queue![];

        add_node(used_nodes, start);
        
        for &i in &board.graph.adj[start as usize].neighbours {
            if self.has_road(start, i as u8, board) {
                q.add(i as u8).unwrap();
                add_node(used_nodes, i as u8);
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
                    moves.push(Move::BuildSettlement(i as u8));
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
                        let mut valid_distance = true;
                        for &neigh in &board.graph.adj[i].neighbours {
                            if self.settlements[neigh as usize] > 0 {
                                valid_distance = false;
                                break;
                            }
                        }

                        if valid_distance {
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
                if self.resources[ENERGY] >= 1 && self.resources[WATER] >= 1 && self.resources[DATA] >=1 && self.resources[RAM] >= 1 {
                    let mut used_nodes = 0;
                    for i in 0..54 {
                        if self.settlements[i] > 0 && used_node(used_nodes, i as u8) {
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

impl Node {

    pub fn new(state: DynamicState, board: &Board) -> Self {
        Self {
            visits: 0,
            current_score: 0.0,
            parent: None,
            untriedActions: state.generate_legal_moves(board),
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
