use std::collections::BTreeMap;
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

#[derive(Debug, Clone)]
struct Room {
    name: Vec<String>,
    id: usize,
    checksum: String,
}

impl Room {
    fn is_valid(self) -> bool {
        let mut checksum = String::new();
        let mut treemap: BTreeMap<char, usize> = BTreeMap::new();
        let mut joined_name = String::new();

        for w in self.name {
            joined_name.push_str(&w)
        }

        // create treemap of characters
        for c in joined_name.chars() {
            treemap.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }

        // get max frecuency of treemap
        let mut max_frecuency = 0;
        for (_, value) in treemap.clone().iter() {
            max_frecuency = max_frecuency.max(*value)
        }

        // println!("max frecuency: {}", max_frecuency);

        // build checksum from treemap
        while checksum.len() < 5 {
            for (key, value) in treemap.iter() {
                if *value == max_frecuency && checksum.len() < 5 {
                    checksum.push(*key);
                }
            }
            max_frecuency -= 1;
        }

        // println!("checksum: {}", checksum);

        if self.checksum == checksum {
            return true;
        }
        false
    }

    fn decrypt_name(self) -> String {
        let mut joined_name = String::new();
        let space = String::from(" ");

        for word in self.name {
            let mut decrypted_word = String::new();

            for char in word.chars() {
                let turned_num = char as u8 + (self.id % 26) as u8;
                let result_char = turned_num as char;
                decrypted_word.push(result_char);
            }

            joined_name.push_str(&decrypted_word);
            joined_name.push_str(&space);
        }

        joined_name
    }
}

pub fn execute() {
    // parse rooms
    let mut rooms: Vec<Room> = vec![];

    if let Ok(lines) = read_lines("input/day4_input.txt") {
        for line in lines {
            if let Ok(line) = line {
                let splited: Vec<&str> = line.split('-').collect();
                if let Some((last, elements)) = splited.split_last() {
                    let mut name: Vec<String> = vec![];
                    for e in elements {
                        name.push(String::from(*e));
                    }
                    let id_and_checksum: Vec<&str> = last.split("[").collect();
                    let id = id_and_checksum[0].parse::<usize>().unwrap_or(0);
                    let checksum = id_and_checksum[1].trim_matches(']');
                    rooms.push(Room {
                        name,
                        id,
                        checksum: checksum.to_owned(),
                    });
                }
            }
        }
    }

    let mut sum_of_valid_rooms = 0;

    for room in rooms {
        let id = room.id.clone();
        if room.clone().is_valid() {
            println!("room name: {} - id: {}", room.decrypt_name(), id);
            sum_of_valid_rooms += id;
        }
    }
    println!("sum of valid rooms: {}", sum_of_valid_rooms);
}
