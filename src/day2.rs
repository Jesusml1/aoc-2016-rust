use std::io::{self, BufRead};
use std::vec;
use std::{fs::File, path::Path};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum Instruction {
    Up,
    Down,
    Right,
    Left,
}

struct Cursor(i32, i32);

impl Cursor {
    fn new() -> Cursor {
        Cursor(1, 1)
    }

    fn update(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Up => {
                if self.0 == 1 || self.0 == 3 {
                    if self.1 > 1 && self.1 <= 3 {
                        self.1 -= 1;
                    }
                }
                if self.0 == 2 {
                    if self.1 > 0 && self.1 <= 4 {
                        self.1 -= 1;
                    }
                }
            }
            Instruction::Down => {
                if self.0 == 1 || self.0 == 3 {
                    if self.1 >= 1 && self.1 < 3 {
                        self.1 += 1;
                    }
                }
                if self.0 == 2 {
                    if self.1 >= 0 && self.1 < 4 {
                        self.1 += 1;
                    }
                }
            }
            Instruction::Right => {
                if self.1 == 1 || self.1 == 3 {
                    if self.0 >= 1 && self.0 < 3 {
                        self.0 += 1;
                    }
                }
                if self.1 == 2 {
                    if self.0 >= 0 && self.0 < 4 {
                        self.0 += 1;
                    }
                }
            }
            Instruction::Left => {
                if self.1 == 1 || self.1 == 3 {
                    if self.0 > 1 && self.0 <= 3 {
                        self.0 -= 1;
                    }
                }
                if self.1 == 2 {
                    if self.0 > 0 && self.0 <= 4 {
                        self.0 -= 1;
                    }
                }
            }
        }
    }

    fn get_numpad_value(&self) -> char {
        match self {
            Cursor(2, 0) => '1',
            Cursor(1, 1) => '2',
            Cursor(2, 1) => '3',
            Cursor(3, 1) => '4',
            Cursor(0, 2) => '5',
            Cursor(1, 2) => '6',
            Cursor(2, 2) => '7',
            Cursor(3, 2) => '8',
            Cursor(4, 2) => '9',
            Cursor(1, 3) => 'A',
            Cursor(2, 3) => 'B',
            Cursor(3, 3) => 'C',
            Cursor(2, 4) => 'D',
            _ => '0',
        }
    }
}

pub fn execute() {
    // parse instructions
    let mut instructions_set: Vec<String> = vec![];

    if let Ok(lines) = read_lines("input/day2_input.txt") {
        for line in lines {
            if let Ok(instruction_set) = line {
                instructions_set.push(instruction_set.clone());
            }
        }
    }

    // process each line and each instruction
    let mut code = String::new();
    let mut cursor = Cursor::new();
    for instruction_set in instructions_set {
        for instruction in instruction_set.chars() {
            let instruction = match instruction {
                'U' => Instruction::Up,
                'D' => Instruction::Down,
                'R' => Instruction::Right,
                'L' => Instruction::Left,
                _ => panic!("invalid instruction"),
            };
            cursor.update(instruction);
        }
        code.push(cursor.get_numpad_value());
    }

    println!("code is: {}", code);
}
