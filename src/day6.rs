use std::collections::HashMap;

use crate::utils;

pub fn execute() {
    let mut input: Vec<String> = vec![];
    let mut columns: Vec<HashMap<char, usize>> = vec![];

    let lines = utils::read_lines("input/day6_input.txt").unwrap();
    for line in lines {
        if let Ok(line) = line {
            input.push(line)
        }
    }

    let columns_number = 8;
    let mut i = 0;

    loop {
        let mut hashmap: HashMap<char, usize> = HashMap::new();

        for word in input.clone() {
            let character = word.get(i..i + 1).unwrap().parse::<char>().unwrap();
            hashmap
                .entry(character)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        i += 1;
        columns.push(hashmap);

        if i >= columns_number {
            break;
        }
    }

    for c in columns {
        let mut v: Vec<(&char, &usize)> = c.iter().collect();
        v.sort_by(|a, b| a.1.cmp(b.1));
        print!("{}", v[0].0);
    }
    println!();
}
