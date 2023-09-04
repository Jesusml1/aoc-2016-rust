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

fn check_triangle(triangle_lens: &Vec<usize>) -> bool {
    let l1 = triangle_lens[0];
    let l2 = triangle_lens[1];
    let l3 = triangle_lens[2];
    if l1 + l2 <= l3 || l2 + l3 <= l1 || l3 + l1 <= l2 {
        return false;
    }
    true
}

pub fn execute() {
    // parse instructions
    let mut triangles_set: Vec<Vec<usize>> = vec![];

    if let Ok(lines) = read_lines("input/day3_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let lens: Vec<usize> = line
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap_or(0))
                    .collect();
                triangles_set.push(lens);
            }
        }
    }

    // check lengths of triangles
    let mut valid_triangles = 0;
    for set in triangles_set.clone() {
        if check_triangle(&set) {
            valid_triangles += 1;
        }
    }

    // check other configuration for part II
    let mut set0: Vec<usize> = vec![];
    let mut set1: Vec<usize> = vec![];
    let mut set2: Vec<usize> = vec![];
    let mut transposed_triangles: Vec<Vec<usize>> = vec![];

    for set in triangles_set {
        set0.push(set[0]);
        set1.push(set[1]);
        set2.push(set[2]);

        if set0.len() > 2 && set1.len() > 2 && set2.len() > 2  {
            transposed_triangles.push(set0.clone());
            transposed_triangles.push(set1.clone());
            transposed_triangles.push(set2.clone());

            set0.clear();
            set1.clear();
            set2.clear();
        }
    }

    let mut valid_transposed_triangles = 0;
    for set in transposed_triangles.clone() {
        if check_triangle(&set) {
            valid_transposed_triangles += 1;
        }
    }

    println!("valid triangles: {}", valid_triangles);
    println!("valid transposed triangles: {}", valid_transposed_triangles);
}
