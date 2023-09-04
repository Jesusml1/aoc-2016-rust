use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    L,
    R,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i32, i32);

#[derive(Debug)]
struct Position {
    point: Point,
    direction: Direction,
    visited_locations: Vec<Point>,
}

impl Position {
    fn new() -> Position {
        Position {
            point: Point(0, 0),
            direction: Direction::North,
            visited_locations: Vec::from([Point(0, 0)]),
        }
    }

    fn update_position(&mut self, turn: Turn, steps: i32) {
        match turn {
            Turn::L => match self.direction {
                Direction::North => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0 - i, self.point.clone().1);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.0 -= steps;
                    self.direction = Direction::West;
                }
                Direction::East => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0, self.point.clone().1 + i);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.1 += steps;
                    self.direction = Direction::North;
                }
                Direction::South => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0 + i, self.point.clone().1);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.0 += steps;
                    self.direction = Direction::East;
                }
                Direction::West => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0, self.point.clone().1 - i);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.1 -= steps;
                    self.direction = Direction::South;
                }
            },
            Turn::R => match self.direction {
                Direction::North => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0 + i, self.point.clone().1);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.0 += steps;
                    self.direction = Direction::East;
                }
                Direction::East => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0, self.point.clone().1 - i);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.1 -= steps;
                    self.direction = Direction::South;
                }
                Direction::South => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0 - i, self.point.clone().1);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.0 -= steps;
                    self.direction = Direction::West;
                }
                Direction::West => {
                    for i in 1..steps {
                        let current_point = Point(self.point.clone().0, self.point.clone().1 + i);
                        if self.visited_locations.contains(&current_point) {
                            println!("already visited: {:?}", current_point);
                        };
                        self.visited_locations.push(current_point);
                    }
                    self.point.1 += steps;
                    self.direction = Direction::North;
                }
            },
        }
    }
}

pub fn execute() {
    // parse the instructions
    let path = Path::new("input/day1_input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open file {}: {}", display, why),
    };

    let mut content: String = String::new();

    file.read_to_string(&mut content).unwrap_or_else(|err| {
        print!("couldn't be read: {}", err);
        std::process::exit(1);
    });

    let instructions: Vec<&str> = content.split_terminator(", ").collect();
    let mut position: Position = Position::new();

    // process each instruction
    for instruction in instructions {
        let (direction, steps) = instruction.split_at(1);

        let turn: Turn = match direction {
            "L" => Turn::L,
            "R" => Turn::R,
            _ => std::process::exit(1),
        };

        let steps: i32 = str::parse(steps).unwrap_or_else(|err| {
            print!("couldn't parse step as i32: {}", err);
            std::process::exit(1);
        });

        position.update_position(turn, steps);
    }

    // calculate shortest taxicab distance
    let shortest_distance: i32 = position.point.0.abs() + position.point.1.abs();

    println!("Shortest distance is {} blocks.", { shortest_distance });

}

