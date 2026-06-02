use clap::Parser;
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};
use tokio::time::{sleep, interval};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod labyrinth;
mod pathfinding;
mod stats;
mod ui;

use labyrinth::Maze;
use pathfinding::{AStar, DStar, Planner};
use stats::Stats;
use ui::UI;

#[derive(Parser, Debug)]
#[command(name = "Dynamic Maze Visualizer", author, version, about = "Визуализатор поиска пути в динамическом лабиринте")]
struct Args {
    #[arg(long, default_value_t = 30)]
    width: usize,
    
    #[arg(long, default_value_t = 20)]
    height: usize,
    
    #[arg(long, default_value = "1,1")]
    start: String,
    
    #[arg(long, default_value = "28,18")]
    end: String,
    
    #[arg(long, default_value = "astar")]
    algo: String,
    
    #[arg(long, default_value_t = 500)]
    update_interval_ms: u64,
    
    #[arg(long, default_value_t = 2)]
    perturb_radius: usize,
    
    #[arg(long, default_value_t = 0)]
    seed: u64,
    
    #[arg(long)]
    maze_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let (sx, sy) = parse_coords(&args.start);
    let (ex, ey) = parse_coords(&args.end);
    
    let mut maze = if let Some(file) = args.maze_file {
        Maze::from_file(&file, (sx, sy), (ex, ey))?
    } else {
        Maze::new(args.width, args.height, (sx, sy), (ex, ey))
    };
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(args.seed);
    
    let planner: Box<dyn Planner> = match args.algo.to_lowercase().as_str() {
        "astar" => Box::new(AStar::new()),
        "dstar" => Box::new(DStar::new()),
        _ => {
            eprintln!("Unknown algorithm: {}, using A*", args.algo);
            Box::new(AStar::new())
        }
    };
    
    let mut stats = Stats::new();
    let mut ui = UI::new()?;
    let mut running = true;
    let mut paused = false;
    let mut update_interval = Duration::from_millis(args.update_interval_ms);
    let mut last_update = Instant::now();
    let time_start = Instant::now();
    
    ui.draw(&maze, None, &stats, paused, &update_interval)?;
    
    while running {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Char(' ') => {
                            paused = !paused;
                            ui.draw(&maze, None, &stats, paused, &update_interval)?;
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            let new_interval = update_interval.as_millis() as u64 / 2;
                            if new_interval >= 50 {
                                update_interval = Duration::from_millis(new_interval);
                                ui.draw(&maze, None, &stats, paused, &update_interval)?;
                            }
                        }
                        KeyCode::Char('-') => {
                            let new_interval = (update_interval.as_millis() as u64 * 2).min(2000);
                            update_interval = Duration::from_millis(new_interval);
                            ui.draw(&maze, None, &stats, paused, &update_interval)?;
                        }
                        KeyCode::Char('r') => {
                            maze.reset_cursor();
                            stats = Stats::new();
                            ui.draw(&maze, None, &stats, paused, &update_interval)?;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        if !paused && last_update.elapsed() >= update_interval {
            // Обновляем лабиринт
            maze.mutate_in_cursor_area(&mut rng, args.perturb_radius);
            
            // Ищем путь
            let path = planner.find_path(
                maze.cursor,
                maze.end,
                &maze.grid,
                maze.width,
                maze.height
            );
            
            // Продвигаемся по пути
            if let Some(path) = &path {
                let progressed = maze.advance_along_path(path);
                stats.log_progress(progressed, path.len());
                
                if progressed > 0 {
                    stats.update_path_changes(path);
                }
                
                ui.draw(&maze, Some(path), &stats, paused, &update_interval)?;
            } else {
                ui.draw(&maze, None, &stats, paused, &update_interval)?;
            }
            
            last_update = Instant::now();
        }
        
        if maze.is_at_end() {
            let duration = time_start.elapsed();
            stats.finish(duration);
            ui.draw(&maze, None, &stats, paused, &update_interval)?;
            
            // Ждем нажатия q для выхода
            while running {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('q') && key.kind == KeyEventKind::Press {
                            running = false;
                            break;
                        }
                    }
                }
            }
        }
        
        sleep(Duration::from_millis(10)).await;
    }
    
    ui.cleanup()?;
    println!("\n{}", stats.summary());
    Ok(())
}

fn parse_coords(s: &str) -> (usize, usize) {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        let x = parts[0].parse().unwrap_or(0);
        let y = parts[1].parse().unwrap_or(0);
        (x, y)
    } else {
        (0, 0)
    }
}
// cargo run -- --width 10 --height 8 --start "1,1" --end "8,6"