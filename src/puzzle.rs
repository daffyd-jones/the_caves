//puzzle.rs
//
use std::collections::HashMap;
use crate::enums::{Cells, NPCWrap, PuzzleType};
//use rand::Rng;
use crate::item::Item;
use crate::enemy::Enemy;
use rand::prelude::SliceRandom;

pub fn make_maze_map() -> Vec<Vec<Cells>> {
    let (m_width, m_height) = (300, 200); 
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![Cells::Wall; m_width]; m_height];
    let mut small_cells = vec![vec![Cells::Wall; 75]; 50];

    fn carve_passages(start_x: usize, start_y: usize, cells: &mut Vec<Vec<Cells>>, rng: &mut rand::rngs::ThreadRng) {
        let mut stack = vec![(start_x, start_y)];
        let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        while let Some((x, y)) = stack.pop() {
            let mut directions = directions.to_vec();
            directions.shuffle(rng);

            for &(dx, dy) in &directions {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                let nnx = nx.wrapping_add(dx as usize);
                let nny = ny.wrapping_add(dy as usize);

                if nnx < 75 && nny < 50 && cells[nny][nnx] == Cells::Wall && cells[ny][nx] == Cells::Wall {
                    cells[y][x] = Cells::Empty;
                    cells[ny][nx] = Cells::Empty;
                    cells[nny][nnx] = Cells::Empty;
                    stack.push((nnx, nny));
                }
            }
        }
    }

    // Start carving from the center of the map
    let start_x = 75 / 2;
    let start_y = 50 / 2;
    carve_passages(start_x, start_y, &mut small_cells, &mut rng);

    for y in 0..50 {
        for x in 0..75 {
            let cell = small_cells[y][x];
            for dy in 0..4 {
                for dx in 0..4 {
                    cells[y * 4 + dy][x * 4 + dx] = cell;
                }
            }
        }
    }
    cells.clone()
}


fn make_maze_mapda() -> Vec<Vec<Cells>> {
    let (m_width, m_height) = (300, 200); 
    let mut rng = rand::thread_rng();
    let mut cells = vec![vec![Cells::Wall; m_width]; m_height];
    //let mut small_cells = vec![vec![Cells::Wall; 150]; 100];
    fn carve_passages(start_x: usize, start_y: usize, cells: &mut Vec<Vec<Cells>>, rng: &mut rand::rngs::ThreadRng) {
        let mut stack = vec![(start_x, start_y)];
        let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        while let Some((x, y)) = stack.pop() {
            let mut directions = directions.to_vec();
            directions.shuffle(rng);
            for &(dx, dy) in &directions {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                let nnx = nx.wrapping_add(dx as usize);
                let nny = ny.wrapping_add(dy as usize);
                if nnx < 150 && nny < 100 && cells[nny][nnx] == Cells::Wall && cells[ny][nx] == Cells::Wall {
                    cells[y][x] = Cells::Empty;
                    cells[ny][nx] = Cells::Empty;
                    cells[nny][nnx] = Cells::Empty;
                    stack.push((nnx, nny));
                }
            }
        }
    }
    // Start carving from the center of the map
    let start_x = m_width / 2;
    let start_y = m_height / 2;
    carve_passages(start_x, start_y, &mut cells, &mut rng);
    cells.clone()
}


#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle {
    ptype: PuzzleType,
    pos: (i64, i64),
    map: Vec<Vec<Cells>>,
    items: HashMap<(usize, usize), Item>,
    enemies: HashMap<(usize, usize), Enemy>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    prize: Item,
}

impl Puzzle {
    pub fn new(
        ptype: PuzzleType,
        pos: (i64, i64),
        map: Vec<Vec<Cells>>, 
        items: HashMap<(usize, usize), Item>, 
        enemies: HashMap<(usize, usize), Enemy>, 
        npcs: HashMap<(usize, usize), NPCWrap>, 
        prize: Item) -> Self {
        Self{
            ptype,
            pos,
            map,
            items, 
            enemies,
            npcs,
            prize, 
        }
    }

    pub fn new_maze(pos: (i64, i64)) -> Self {
        let map = make_maze_map();
        let items = HashMap::new();
        let enemies = HashMap::new();
        let npcs = HashMap::new();
        let prize = Item::new_health_potion(150, 100);
        Self{
            ptype: PuzzleType::Maze,
            pos,
            map,
            items, 
            enemies,
            npcs,
            prize, 
        }
    }

    pub fn get_pos(&mut self) -> (i64, i64) {
        self.pos.clone()
    }
 
    pub fn set_pos(&mut self, tpos: (i64, i64)) {
        self.pos = tpos;
    }

    pub fn get_map(&mut self) -> Vec<Vec<Cells>> {
        self.map.clone()
    }

}
