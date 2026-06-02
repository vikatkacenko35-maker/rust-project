use rand::Rng;
use std::fs;

pub type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Open,
    Wall,
    Start,
    End,
    Path,
    Cursor,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
    pub start: Point,
    pub end: Point,
    pub cursor: Point,
}

impl Maze {
    pub fn new(width: usize, height: usize, start: Point, end: Point) -> Self {
        let mut grid = vec![vec![Cell::Open; width]; height];
        
        // Стены по краям
        for x in 0..width {
            grid[0][x] = Cell::Wall;
            grid[height - 1][x] = Cell::Wall;
        }
        for y in 0..height {
            grid[y][0] = Cell::Wall;
            grid[y][width - 1] = Cell::Wall;
        }
        
        // Генерация случайных стен (20% заполнение)
        let mut rng = rand::thread_rng();
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.gen_bool(0.2) {
                    grid[y][x] = Cell::Wall;
                }
            }
        }
        
        grid[start.1][start.0] = Cell::Start;
        grid[end.1][end.0] = Cell::End;
        
        Maze {
            width,
            height,
            grid,
            start,
            end,
            cursor: start,
        }
    }
    
    pub fn from_file(filename: &str, start: Point, end: Point) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        let lines: Vec<&str> = content.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        
        let mut grid = vec![vec![Cell::Open; width]; height];
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid[y][x] = match ch {
                    '#' => Cell::Wall,
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    _ => Cell::Open,
                };
            }
        }
        
        grid[start.1][start.0] = Cell::Start;
        grid[end.1][end.0] = Cell::End;
        
        Ok(Maze {
            width,
            height,
            grid,
            start,
            end,
            cursor: start,
        })
    }
    
    pub fn mutate_in_cursor_area<R: Rng>(&mut self, rng: &mut R, radius: usize) {
        let (cx, cy) = self.cursor;
        
        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                
                if nx > 0 && ny > 0 && nx < (self.width - 1) as isize && ny < (self.height - 1) as isize {
                    let x = nx as usize;
                    let y = ny as usize;
                    
                    // Не изменяем старт и финиш
                    if (x, y) != self.start && (x, y) != self.end {
                        if rng.gen_bool(0.4) {
                            self.grid[y][x] = if self.grid[y][x] == Cell::Wall {
                                Cell::Open
                            } else if self.grid[y][x] == Cell::Open {
                                Cell::Wall
                            } else {
                                self.grid[y][x]
                            };
                        }
                    }
                }
            }
        }
    }
    
    pub fn advance_along_path(&mut self, path: &[Point]) -> usize {
        if path.len() <= 1 {
            return 0;
        }
        
        // Ищем следующую позицию курсора
        if let Some(next_pos) = path.get(1) {
            if self.is_walkable(*next_pos) {
                // Очищаем старую позицию курсора
                if self.grid[self.cursor.1][self.cursor.0] == Cell::Cursor {
                    self.grid[self.cursor.1][self.cursor.0] = Cell::Path;
                }
                
                self.cursor = *next_pos;
                
                // Отмечаем новую позицию курсора
                if self.grid[self.cursor.1][self.cursor.0] != Cell::End {
                    self.grid[self.cursor.1][self.cursor.0] = Cell::Cursor;
                }
                return 1;
            }
        }
        0
    }
    
    fn is_walkable(&self, pos: Point) -> bool {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return false;
        }
        match self.grid[y][x] {
            Cell::Open | Cell::Path | Cell::End => true,
            _ => false,
        }
    }
    
    pub fn is_at_end(&self) -> bool {
        self.cursor == self.end
    }
    
    pub fn reset_cursor(&mut self) {
        // Очищаем следы пути
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == Cell::Path || self.grid[y][x] == Cell::Cursor {
                    self.grid[y][x] = Cell::Open;
                }
            }
        }
        self.cursor = self.start;
        self.grid[self.start.1][self.start.0] = Cell::Start;
    }
}