use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::labyrinth::Cell;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Planner {
    fn find_path(&self, start: (usize, usize), end: (usize, usize),
                 grid: &Vec<Vec<Cell>>, width: usize, height: usize) -> Option<Vec<(usize, usize)>>;
}

pub struct AStar;

impl AStar {
    pub fn new() -> Self {
        AStar
    }
    
    fn heuristic(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let (ax, ay) = a;
        let (bx, by) = b;
        ((ax as isize - bx as isize).abs() + (ay as isize - by as isize).abs()) as usize
    }
}

impl Planner for AStar {
    fn find_path(&self, start: (usize, usize), end: (usize, usize),
                 grid: &Vec<Vec<Cell>>, width: usize, height: usize) -> Option<Vec<(usize, usize)>> {
        
        let mut came_from = std::collections::HashMap::new();
        let mut cost_so_far = std::collections::HashMap::new();
        let mut heap = BinaryHeap::new();
        
        heap.push(State {
            cost: 0,
            position: start,
        });
        cost_so_far.insert(start, 0);
        
        while let Some(State { cost, position }) = heap.pop() {
            if position == end {
                let mut path = vec![position];
                let mut current = position;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                return Some(path);
            }
            
            if cost > *cost_so_far.get(&position).unwrap_or(&usize::MAX) {
                continue;
            }
            
            let (x, y) = position;
            let neighbors = [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];
            
            for (nx, ny) in neighbors {
                if nx < width && ny < height {
                    if let Cell::Wall = grid[ny][nx] {
                        continue;
                    }
                    
                    let new_cost = cost_so_far[&position] + 1;
                    let next = (nx, ny);
                    
                    if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                        cost_so_far.insert(next, new_cost);
                        let priority = new_cost + self.heuristic(next, end);
                        heap.push(State {
                            cost: priority,
                            position: next,
                        });
                        came_from.insert(next, position);
                    }
                }
            }
        }
        
        None
    }
}

pub struct DStar;

impl DStar {
    pub fn new() -> Self {
        DStar
    }
}

impl Planner for DStar {
    fn find_path(&self, start: (usize, usize), end: (usize, usize),
                 grid: &Vec<Vec<Cell>>, width: usize, height: usize) -> Option<Vec<(usize, usize)>> {
        
        // Упрощенная реализация D* (фактически A* для статического графа)
        // В полной версии D* должен перепланировать путь при изменениях
        let astar = AStar::new();
        astar.find_path(start, end, grid, width, height)
    }
}