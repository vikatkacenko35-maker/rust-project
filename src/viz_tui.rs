use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crate::labyrinth::{Cell, Maze};
use crate::pathfinding::Planner;

pub fn clear_screen() {
    // Crossterm-очистка через ANSI-escape
    use crossterm::terminal::{Clear, ClearType};
    use std::io::{stdout, Write};
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().flush().unwrap();
}

pub fn print_maze(maze: &Maze, path: &Vec<(usize, usize)>, terminal: &mut Terminal<crate::viz_tui::CrosstermBackendWrapper>) {
    // Простейшая печать через Paragraph — адаптируйте под Canvas чтобы было реже
    // Здесь демонстрационный подход: собираем текстовую сетку
    let mut lines = Vec::new();
    for y in 0..maze.height {
        let mut line = String::new();
        for x in 0..maze.width {
            let pos = (x, y);
            let is_cursor = pos == maze.cursor;
            let mut ch = match maze.grid[y][x] {
                Cell::Open => " . ".to_string(),
                Cell::Blocked => " █".to_string(),
                Cell::Start => " S".to_string(),
                Cell::End => " E".to_string(),
                Cell::Path => " •".to_string(),
            };
            if path.contains(&pos) && maze.grid[y][x] != Cell::Start && maze.grid[y][x] != Cell::End {
                ch = " •".to_string();
            }
            if is_cursor {
                ch = "○".to_string();
            }
            line.push_str(&ch);
        }
        lines.push(line);
    }
    let text = lines.join("\n");
    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Dynamic Maze"));
    // здесь упрощение: используем фактически обычный println через фрейм
    // Реализация через ratatui требует корректной раскладки, но демонстрация дана для идеи
    // В продакшн-версии рисуем через terminal.draw(...)
    // По умолчанию выводим через stdout:
    println!("{}", text);
}