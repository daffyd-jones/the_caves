use crate::enums::NodeType;
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
    pub id: String,
    pub ntype: NodeType,
    pub pos: (i64, i64),
    pub name: String,
    pub neighbors: Vec<String>,
    pub neighbor_ids: Vec<String>,
}

pub struct NodeMap {
    depth: u16,
    ul_nodes: HashMap<u16, Vec<Node>>,
    ur_nodes: HashMap<u16, Vec<Node>>,
    dl_nodes: HashMap<u16, Vec<Node>>,
    dr_nodes: HashMap<u16, Vec<Node>>,
}

impl NodeMap {
    pub fn new() -> Self {
        let ul_nodes = HashMap::new();
        let ur_nodes = HashMap::new();
        let dl_nodes = HashMap::new();
        let dr_nodes = HashMap::new();
        NodeMap {
            depth: 0,
            ul_nodes,
            ur_nodes,
            dl_nodes,
            dr_nodes,
        }
    }

    pub fn increase_depth(&mut self, quad: &str) -> Vec<Node> {
        let data1 = fs::read_to_string("src/locations/settle_names.json");
        let names: Vec<String> = match data1 {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(e) => {
                log::info!("{:?}", e);
                Vec::new()
            }
        };
        let depth = self.depth + 1;
        self.depth = depth;
        let mut rng = rand::thread_rng();
        let mut d_nodes = Vec::new();
        let (mut nodes, dir) = match quad {
            "ul" => (self.ul_nodes.clone(), (-1, -1)),
            "ur" => (self.ur_nodes.clone(), (1, -1)),
            "dl" => (self.dl_nodes.clone(), (-1, 1)),
            "dr" => (self.dr_nodes.clone(), (1, 1)),
            _ => todo!(),
        };
        let base = {
            match depth % 2 {
                0 => 0,
                1 => 800,
                _ => todo!(),
            }
        };
        let mut id_cnt = 0;
        for i in (base..(800 * depth) + 1).step_by(1600) {
            let pos = if i == (800 * depth) {
                let xpoint = rng.gen_range(i - 100..i + 100);
                let ypoint = rng.gen_range(i - 100..i + 100);
                ((xpoint as i64 * dir.0), (ypoint as i64 * dir.1))
                // println!("{:?}", point);
                // continue;
            } else {
                // let y = (depth * 800, i);
                let x = (depth * 800, i);
                let xpoint = rng.gen_range(x.0 - 100..x.0 + 100);
                let ypoint = match i {
                    0 => rng.gen_range(x.1..x.1 + 100),
                    _ => rng.gen_range(x.1 - 100..x.1 + 100),
                };
                ((xpoint as i64 * dir.0), (ypoint as i64 * dir.1))
                // (xpoint as i64, ypoint as i64)
            };

            // let ntype = NodeType::Settlement;
            let ntype = *[NodeType::Settlement, NodeType::Puzzle, NodeType::Null]
                .choose(&mut rng)
                .unwrap_or(&NodeType::Settlement);
            let name = if ntype == NodeType::Settlement {
                let mut rng = rand::thread_rng();
                let name_oops = "Jadeitite".to_string();
                names.choose(&mut rng).unwrap_or(&name_oops.clone()).clone()
            } else {
                "".to_string()
            };

            // let name = "Quartz".to_string();
            let neighbors = Vec::new();
            let neighbor_ids = Vec::new();
            // let id = "1".to_string();
            let id = format!("{}|{}", depth, id_cnt);

            d_nodes.push(Node {
                id,
                ntype,
                pos,
                name,
                neighbors,
                neighbor_ids,
            });

            id_cnt += 1;

            if i != 0 && pos.0 != pos.1 {
                let x = (depth * 800, i);
                let xpoint = rng.gen_range(x.0 - 100..x.0 + 100);
                let ypoint = rng.gen_range(x.1 - 100..x.1 + 100);
                let pos = ((xpoint as i64 * dir.0), (ypoint as i64 * dir.1));
                // (xpoint as i64, ypoint as i64)

                let ntype = *[NodeType::Settlement, NodeType::Puzzle, NodeType::Null]
                    .choose(&mut rng)
                    .unwrap_or(&NodeType::Settlement);
                // let name = "Quartz".to_string();
                let name = if ntype == NodeType::Settlement {
                    let mut rng = rand::thread_rng();
                    let name_oops = "Jadeitite".to_string();
                    names.choose(&mut rng).unwrap_or(&name_oops.clone()).clone()
                } else {
                    "".to_string()
                };
                let neighbors = Vec::new();
                let neighbor_ids = Vec::new();
                let id = format!("{}|{}", depth, id_cnt);

                d_nodes.push(Node {
                    id,
                    ntype,
                    pos,
                    name,
                    neighbors,
                    neighbor_ids,
                });
                id_cnt += 1;
            }
        }
        nodes.insert(depth, d_nodes.clone());
        match quad {
            "ul" => self.ul_nodes = nodes,
            "ur" => self.ur_nodes = nodes,
            "dl" => self.dl_nodes = nodes,
            "dr" => self.dr_nodes = nodes,
            _ => todo!(),
        };
        d_nodes
    }

    // fn new_node(
    //     &mut self,
    //     quad: &str,
    //     ntype: NodeType,
    //     pos: (i64, i64),
    //     name: String,
    //     rules: HashMap<NodeType, (u16, u16)>,
    //     neighbors: Vec<u16>,
    // ) {

    //     let id = (self.nodes.len() + 1) as u16;
    //     self.nodes.push(Node {
    //         id,
    //         ntype,
    //         pos,
    //         name,
    //         rules,
    //         neighbors,
    //     });
    // }

    // pub fn place_new_settlement(&mut self, pos: (i64, i64)) -> (i64, i64) {
    //     let closest = self
    //         .nodes
    //         .into_iter()
    //         .filter(|x| (x.pos.0 - pos.0).pow(2) + (x.pos.1 - pos.1).pow(2) < 1000)
    //         .collect();

    //     let new_pos = (0, 0);
    //     let rules = HashMap::new();
    //     rules.insert(NodeType::Settlement, (900, 1200));
    //     rules.insert(NodeType::Puzzle, (800, 1200));
    //     rules.insert(NodeType::Feature, (380, 520));

    //     let neighbors = Vec::new();
    //     for n in closest {}

    //     self.new_node(
    //         NodeType::Settlement,
    //         new_pos,
    //         "tnme".to_string(),
    //         rules,
    //         neighbors,
    //     );
    //     new_pos
    // }
}
