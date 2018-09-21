extern crate termion;

use std::{io::{self, Read, Write}, num};

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

}

enum Cell {
    Empty,
    Number(u16)
}
