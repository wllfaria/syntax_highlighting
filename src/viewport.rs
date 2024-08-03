use std::io::{stdout, Write};

use crossterm::style::Color;
use crossterm::*;

#[derive(Debug, Clone)]
pub struct Cell {
    symbol: char,
    fg: Color,
    bg: Color,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Debug)]
pub struct Viewport {
    buffer: Vec<Cell>,
    pub size: (usize, usize),
}

impl Viewport {
    pub fn new() -> Viewport {
        let (cols, rows) = terminal::size().unwrap();
        Viewport {
            buffer: vec![Default::default(); (cols * rows) as usize],
            size: (cols as usize, rows as usize),
        }
    }

    pub fn set_cell(&mut self, col: usize, row: usize, symbol: char) {
        let pos = row * self.size.0 + col;
        self.buffer[pos] = Cell {
            symbol,
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }

    pub fn fill<T, U>(&mut self, mut code: T)
    where
        T: Iterator<Item = U>,
        U: AsRef<str>,
    {
        for row in 0..self.size.1 {
            let line = code.next();
            for col in 0..self.size.0 {
                let symbol = match line {
                    Some(ref l) => l.as_ref().chars().nth(col).unwrap_or(' '),
                    None => ' ',
                };
                let symbol = match symbol {
                    s if s.is_whitespace() => ' ',
                    s => s,
                };
                self.set_cell(col, row, symbol);
            }
        }
    }

    pub fn render(&self) {
        for (idx, cell) in self.buffer.iter().enumerate() {
            let row = idx / self.size.0;
            let col = idx % self.size.0;
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(col as u16, row as u16),
                crossterm::style::Print(cell.symbol)
            )
            .expect("Failed to queue events to stdout");
        }
        stdout().flush().expect("Failed to flush stdout");
    }
}
