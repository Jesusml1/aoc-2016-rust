use crate::utils;
// use std::{thread, time};

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

// const SCREEN_WIDTH: usize = 7;
// const SCREEN_HEIGHT: usize = 3;

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
    active: bool,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Position>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let mut cells: Vec<Position> = Vec::new();
        for i in 0..width {
            for j in 0..height {
                cells.push(Position {
                    x: i,
                    y: j,
                    active: false,
                })
            }
        }
        Grid {
            height,
            width,
            cells,
        }
    }

    fn rect(&mut self, a: usize, b: usize) {
        // for position in &mut self.cells {
        //     if position.y < a && position.x < b {
        //         position.active = true;
        //     }
        // }
        for i in 0..b {
            for j in 0..a {
                self.cells.push(Position { x: i, y: j, active: true });
            }
        }
    }

    fn rotate_row(&mut self, a: usize, b: usize) {
        for position in &mut self.cells {
            if position.x == a {
                let mut moves_left = b;
                while moves_left != 0 {
                    if position.y == self.width - 1 {
                        position.y = 0;
                    } else {
                        position.y += 1;
                    }
                    moves_left -= 1;
                }
            }
        }
    }

    fn rotate_col(&mut self, a: usize, b: usize) {
        for position in &mut self.cells {
            if position.y == a {
                let mut moves_left = b;
                while moves_left != 0 {
                    if position.x == self.height - 1 {
                        position.x = 0;
                    } else {
                        position.x += 1;
                    }
                    moves_left -= 1;
                }
            }
        }
    }

    fn print(&self) {
        let mut canvas: Vec<Vec<usize>> = Vec::new();
        let mut pixels_on: usize = 0;
        for _ in 0..self.height {
            let  row: Vec<usize> = vec![0;self.width];
            canvas.push(row);
        }

        for position in &self.cells {
            if position.active {
                canvas[position.x][position.y] = 1;
            }
        }

        for (i, _) in canvas.iter().take(self.height).enumerate() {
            for j in 0..self.width {
                if canvas[i][j] == 1 {
                    pixels_on += 1;
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("pixels on: {}", pixels_on);
        println!();
    }
}

pub fn execute() {
    // let instructions: Vec<String> = Vec::from([
    //     "rect 3x2".to_string(),
    //     "rotate column x=1 by 1".to_string(),
    //     "rotate row y=0 by 4".to_string(),
    //     "rotate column x=1 by 1".to_string(),
    //     "rect 1x1".to_string(),
    // ]);

    let mut instructions: Vec<String> = Vec::new();
    let lines = utils::read_lines("input/day8_input.txt").unwrap();
    for line in lines.flatten() {
        instructions.push(line);
    }

    // let some_time = time::Duration::from_millis(100);

    let mut grid = Grid::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    for instruction in instructions {
        if instruction.contains("rect") {
            let i: Vec<&str> = instruction.split(&[' ', 'x']).collect();
            let a = i[1].parse::<usize>().expect("error parsing &str to usize");
            let b = i[2].parse::<usize>().expect("error parsing &str to usize");
            grid.rect(a, b);
        } else if instruction.contains("column") || instruction.contains("row") {
            let i: Vec<&str> = instruction.split(&[' ', '=']).collect();
            let a = i[3].parse::<usize>().expect("error parsing &str to usize");
            let b = i[5].parse::<usize>().expect("error parsing &str to usize");
            if instruction.contains("column") {
                grid.rotate_col(a, b);
            } else if instruction.contains("row") {
                grid.rotate_row(a, b);
            }
        }
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // println!("{}", instruction);
        // grid.print();
        // thread::sleep(some_time);
    }

    grid.print();
}