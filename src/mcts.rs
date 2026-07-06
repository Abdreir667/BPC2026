use crate::reader::*;

struct Settlement{
    level: u8,
    node: u8,
}

pub struct Board {
    graph: Graph,
    convertors: Vec<Convertor>
}

pub enum Move {
    buildSettlement(u8),
    upgradeSettlement(u8),
    buildRoad(u8, u8),
    endTurn,
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
}

pub enum GamePhase { 
    SetupCity1, 
    SetupRoad1(u8), 
    SetupCity2, 
    SetupRoad2(u8),
    NormalState,
}

impl DynamicState {

    pub fn new(self) -> Self {
        Self {
            turn_number : 0, 
            points: 0, 
            resources: [0; 5], 
            settlements: [0; 54], 
            roads: [[0; 2]; 54], 
            phase: GamePhase::SetupCity1
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
                            moves.push(Move::buildSettlement(i as u8));
                        }
                    }
                }
            }
            
            GamePhase::SetupRoad1(start_idx) | GamePhase::SetupRoad2(start_idx) => {
                for i in &board.graph.adj[start_idx as usize].neighbours {
                    moves.push(Move::buildRoad(start_idx, *i as u8));
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