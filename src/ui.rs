use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};
use std::time::Duration;
use crate::labyrinth::{Maze, Cell, Point};
use crate::stats::Stats;

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
    pub fn new() -> Result<Self, io::Error> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide, Clear(ClearType::All))?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(UI { terminal })
    }
    
    pub fn draw(&mut self, maze: &Maze, path: Option<&Vec<Point>>, stats: &Stats, 
                paused: bool, update_interval: &Duration) -> Result<(), io::Error> {
        self.terminal.draw(|f: &mut Frame<'_>| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(3), Constraint::Length(7)].as_ref())
                .split(f.size());
            
            Self::draw_maze(f, chunks[0], maze, path);
            Self::draw_info(f, chunks[1], maze, stats, paused, update_interval);
        })?;
        Ok(())
    }
    
   fn draw_maze(f: &mut Frame, area: Rect, maze: &Maze, path: Option<&Vec<Point>>) {
    let mut lines = Vec::new();
    
    for y in 0..maze.height {
        let mut spans = Vec::new();
        
        for x in 0..maze.width {
            let pos = (x, y);
            let is_in_path = path.map_or(false, |p| p.contains(&pos));
            let is_next_in_path = path.map_or(false, |p| p.len() > 1 && p[1] == pos);
            
            let (ch, color) = if pos == maze.cursor {
                ('@', Color::Yellow)  // Текущая позиция
            } else if pos == maze.start {
                ('S', Color::Green)   // Старт
            } else if pos == maze.end {
                ('E', Color::Red)     // Финиш
            } else if is_in_path && !is_next_in_path && maze.grid[y][x] != Cell::Path {
                // Показать планируемый путь
                ('·', Color::Cyan)
            } else if is_next_in_path {
                ('→', Color::LightGreen)  // Следующий шаг
            } else {
                match maze.grid[y][x] {
                    Cell::Open => ('.', Color::White),
                    Cell::Wall => ('#', Color::DarkGray),
                    Cell::Path => ('·', Color::Blue),  // Пройденный путь
                    Cell::Cursor => ('@', Color::Yellow),
                    Cell::Start => ('S', Color::Green),
                    Cell::End => ('E', Color::Red),
                }
            };
            
            spans.push(Span::styled(format!("{} ", ch), Style::default().fg(color)));
        }
        
        lines.push(Line::from(spans));
    }
    
    let maze_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Лабиринт"));
    
    f.render_widget(maze_widget, area);
}
    fn draw_info(f: &mut Frame, area: Rect, maze: &Maze, stats: &Stats,
                 paused: bool, update_interval: &Duration) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);
        
        let status_text = if paused { "⏸ ПАУЗА" } else { "▶ ЗАПУЩЕН" };
        let speed = format!("Скорость: {:.1} обн/сек", 1000.0 / update_interval.as_millis() as f64);
        
        let info_text = vec![
            Line::from(Span::styled(
                format!("Статус: {}", status_text),
                Style::default().fg(if paused { Color::Yellow } else { Color::Green })
            )),
            Line::from(Span::raw(format!("Позиция: ({}, {})", maze.cursor.0, maze.cursor.1))),
            Line::from(Span::raw(format!("Шагов сделано: {}", stats.steps()))), // Используем геттер
            Line::from(Span::raw(speed)),
        ];
        
        let controls_text = vec![
            Line::from(Span::raw("Управление:")),
            Line::from(Span::raw("  Space - Пауза/Запуск")),
            Line::from(Span::raw("  +/- - Скорость")),
            Line::from(Span::raw("  R - Сброс")),
            Line::from(Span::raw("  Q - Выход")),
        ];
        
        let info_widget = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Информация"));
        
        let controls_widget = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::ALL).title("Управление"));
        
        f.render_widget(info_widget, chunks[0]);
        f.render_widget(controls_widget, chunks[1]);
    }
    
    pub fn cleanup(&mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            cursor::Show,
            Clear(ClearType::All)
        )?;
        self.terminal.clear()?;
        Ok(())
    }
}