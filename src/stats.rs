use std::time::Duration;
use crate::labyrinth::Point;

pub struct Stats {
    steps: usize,
    total_path_length: usize,
    path_changes: usize,
    dead_ends_visited: usize,
    duration: Option<Duration>,
    last_path: Option<Vec<Point>>,
    valid_steps: usize,
    total_steps: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            steps: 0,
            total_path_length: 0,
            path_changes: 0,
            dead_ends_visited: 0,
            duration: None,
            last_path: None,
            valid_steps: 0,
            total_steps: 0,
        }
    }
    pub fn steps(&self) -> usize {
        self.steps
    }
    
    pub fn total_path_length(&self) -> usize {
        self.total_path_length
    }
    
    pub fn path_changes(&self) -> usize {
        self.path_changes
    }
    
    pub fn dead_ends_visited(&self) -> usize {
        self.dead_ends_visited
    }
    
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
    
    pub fn valid_steps(&self) -> usize {
        self.valid_steps
    }
    
    pub fn total_steps(&self) -> usize {
        self.total_steps
    }
    
    pub fn log_progress(&mut self, delta: usize, path_length: usize) {
        self.steps += delta;
        self.total_steps += 1;
        
        if delta > 0 {
            self.valid_steps += 1;
        }
    }
    
    pub fn update_path_changes(&mut self, new_path: &[Point]) {
        if let Some(ref last_path) = self.last_path {
            if last_path != new_path {
                self.path_changes += 1;
            }
        }
        self.last_path = Some(new_path.to_vec());
        self.total_path_length = new_path.len();
    }
    
    pub fn finish(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }
    
    pub fn summary(&self) -> String {
        let duration = self.duration.map(|d| d.as_secs_f64()).unwrap_or(0.0);
        let path_validity = if self.total_steps > 0 {
            (self.valid_steps as f64 / self.total_steps as f64) * 100.0
        } else {
            0.0
        };
        
        format!("\n=== ОТЧЕТ О ПОИСКЕ ПУТИ ===\n\
                Фактическая длина маршрута: {}\n\
                Количество перерасчетов маршрута: {}\n\
                Время достижения цели: {:.3} сек\n\
                Стабильность маршрута: {:.1}%\n\
                Валидность пути: {:.1}%\n\
                ==============================",
            self.total_path_length,
            self.path_changes,
            duration,
            if self.path_changes > 0 { 
                (1.0 - (self.path_changes as f64 / self.total_steps as f64).min(1.0)) * 100.0 
            } else { 
                100.0 
            },
            path_validity
        )
    }
}