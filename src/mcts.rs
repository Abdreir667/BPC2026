use crate::reader::*;

pub struct Board {
    pub graph: Graph,
    pub convertors: Vec<Convertor>,
    pub edge_id: [[u8; 54]; 54],
}

#[derive(Clone, Copy)]
pub enum Resource {
    Energy, 
    Water,
    Data, 
    Ram, 
    Gpu,
}

#[derive(Clone, Copy)]
pub enum Move {
    BuildSettlement(u8),
    UpgradeSettlement(u8),
    BuildRoad(u8, u8),
    EndTurn,
    Trade(u8, u8, u8),
}

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
    builtRoads: u128,
}

pub enum GamePhase { 
    SetupCity1, 
    SetupRoad1(u8), 
    SetupCity2, 
    SetupRoad2(u8),
    NormalState,
}

impl DynamicState {

    pub fn has_road(&self, u: u8, v: u8, board: &Board) -> bool {
        let edge_id = board.edge_id[u as usize][v as usize];
        if edge_id == 254 {
            false
        } else {
            self.builtRoads & (1 << edge_id) != 0
        }
    }

    pub fn apply_move(&mut self, action: Move, board: &Board) {
        match action {
            Move::BuildRoad(u, v) => {
                let edge_id = board.edge_id[u as usize][v as usize]; 
                self.builtRoads |= 1 << edge_id;
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
            builtRoads: 0,
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