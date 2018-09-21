extern crate termion;

use std::{io::{self, Read, Write}, num, collections};

use termion::{cursor, raw::IntoRawMode};

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let size: (u16, u16) = termion::terminal_size().unwrap();

    let (height, width) = size;

    //so the size of the grid should be the largest multiple of 4x that is less than or equal both the height and the width;


    write!(stdout, "{}\n{}\n", height, width);
}

fn size_of_cell(width: u16, height: u16) -> u16 {
    if height < width {
        height / 4
    } else {
        width / 4
    }
}

struct Grid {
    inner: [[Cell; 4]; 4]
}

impl Grid {
    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                let mut queue = collections::VecDeque::new();
                for i in 0..self.inner[0].len() {
                    let row = &self.inner[0];
                    let current = row[i];
                    if current == Cell::Empty {
                        queue.push_back(i)
                    } else {

                    }
                }
            },
            Direction::Down => ,
            Direction::Left => ,
            Direction::Right => ,
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Number(u16)
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}
