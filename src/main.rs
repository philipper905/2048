extern crate termion;
extern crate rand;

use std::{io::{self, Read, Write}, num, collections, mem};

use termion::{cursor, raw::IntoRawMode};
use rand::Rng;

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
        let mut queue = collections::VecDeque::new();

        match dir {
            Direction::Left => {

                for y in 0..self.inner.len() {
                    for x in 0..self.inner[y].len() {

                        let current = self.inner[y][x];
                        if current == Cell::Empty {
                            queue.push_back(x)
                        } else {
                            if let Some(open_index) = queue.pop_front() {
                                self.inner[y][x] = Cell::Empty;
                                self.inner[y][open_index] = current;
//                                self.inner[j].swap(open_index, i)
                            }
                        }
                    }
                }
            },
            Direction::Down => {
                for x in 0..self.inner.len() {
                    for y in 0..self.inner.len() {
                        let current = self.inner[y][x];
                        if current == Cell::Empty {
                            queue.push_back(y)
                        } else {
                            if let Some(open_index) = queue.pop_front() {
//                                let intermediate = self.inner[j][open_index];
                                self.inner[y][x] = Cell::Empty;
                                self.inner[open_index][x] = current;
                            }
                        }
                    }
                }
            },
            Direction::Up => {
                for x in 0..self.inner.len() {
                    for y in (0..self.inner.len()).rev() {
                        let current = self.inner[y][x];
                        if current == Cell::Empty {
                            queue.push_back(y)
                        } else {
                            if let Some(open_index) = queue.pop_front() {
//                                let intermediate = self.inner[j][open_index];
                                self.inner[y][x] = Cell::Empty;
                                self.inner[open_index][x] = current;
                            }
                        }
                    }
                }
            },
            Direction::Right => {
                for y in 0..self.inner.len() {
                    for x in (0..self.inner[y].len()).rev() {
                        let current = self.inner[y][x];
                        if current == Cell::Empty {
                            queue.push_back(x)
                        } else {
                            if let Some(open_index) = queue.pop_front() {
                                self.inner[y][x] = Cell::Empty;
                                self.inner[y][open_index] = current;
//                                self.inner[j].swap(open_index, i)
                            }
                        }
                    }
                }
            },
        }
    }

    fn spawn_block(&mut self) -> bool {
        let rng = rand::thread_rng();

        let value: u16 = 2; // should be 2 or 4

        let len = self.inner.len();
        let all_spots = (0..len).map(|y| {
            (0..len).map(|x| {
                (x, y)
            })
        });

        let open_spots: Vec<(usize, usize)> = all_spots.filter(|(x, y)| {
            self.inner[y][x] == Cell::Empty
        }).collect();

        let rand_open_spot = open_spots.choose();
        if let Some((x, y)) = rand_open_spot {

            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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
