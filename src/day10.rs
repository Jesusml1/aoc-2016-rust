// use std::collections::BTreeMap;
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::utils;

#[derive(Clone, Debug)]
struct Instruction {
    line: String,
    executed: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Chips(usize, usize);

pub fn execute() {
    // let sample_instructions: Vec<String> = vec![
    //     "value 5 goes to bot 2".to_string(),
    //     "bot 2 gives low to bot 1 and high to bot 0".to_string(),
    //     "value 3 goes to bot 1".to_string(),
    //     "bot 1 gives low to output 1 and high to bot 0".to_string(),
    //     "bot 0 gives low to output 2 and high to output 0".to_string(),
    //     "value 2 goes to bot 2".to_string(),
    // ];

    // parsing instructions
    let mut instructions: Vec<Instruction> = Vec::new();

    // for si in sample_instructions {
    //     instructions.push(Instruction {
    //         line: si,
    //         executed: false,
    //     });
    // }

    let lines = utils::read_lines("input/day10_input.txt").unwrap();
    for line in lines.flatten() {
        instructions.push(Instruction {
            line,
            executed: false,
        });
    }

    let mut bots: HashMap<usize, Chips> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    loop {
        for instruction in &mut instructions {
            if instruction.executed {
                continue;
            }

            if instruction.line.contains("value") {
                let ins_vec: Vec<&str> = instruction.line.split_whitespace().collect();
                let value = ins_vec[1].parse::<usize>().unwrap();
                let bot_id = ins_vec[5].parse::<usize>().unwrap();

                if let Some(chips) = bots.get_mut(&bot_id) {
                    if value > chips.1 {
                        chips.0 = chips.1;
                        chips.1 = value;
                    } else {
                        chips.0 = value;
                    }
                } else {
                    bots.insert(bot_id, Chips(0, value));
                }

                instruction.executed = true;
            }

            //
            // parsing bot transactions
            //
            if instruction.line.contains("gives") {
                // parse instructions
                let ins_vec: Vec<&str> = instruction.line.split_whitespace().collect();

                let from_bot_id = ins_vec[1].parse::<usize>().unwrap();
                // check if bot has both chips
                if let Some(chips) = bots.get(&from_bot_id) {
                    if chips.0 == 0 || chips.1 == 0 {
                        continue;
                    } else {
                        if chips.0 == 17 && chips.1 == 61 {
                            println!("id: {}", from_bot_id);
                        }
                    }
                } else {
                    continue;
                }

                let low_to_type = ins_vec[5];
                let low_to_id = ins_vec[6].parse::<usize>().unwrap();
                if low_to_type == "bot" {
                    bots.entry(low_to_id).or_insert(Chips(0, 0));
                }

                let high_to_type = ins_vec[10];
                let high_to_id = ins_vec[11].parse::<usize>().unwrap();
                if high_to_type == "bot" {
                    bots.entry(high_to_id).or_insert(Chips(0, 0));
                }

                // make transactions
                if let Some(chips) = bots.get(&from_bot_id) {
                    let low = chips.0;
                    let high = chips.1;

                    // low
                    if low_to_type == "bot" {
                        if let Some(chips) = bots.get_mut(&low_to_id) {
                            if low > chips.1 {
                                chips.0 = chips.1;
                                chips.1 = low;
                            } else {
                                chips.0 = low;
                            }
                        }
                    } else if low_to_type == "output" {
                        outputs
                            .entry(low_to_id)
                            .and_modify(|o| *o = low)
                            .or_insert(low);
                    }

                    //high
                    if high_to_type == "bot" {
                        if let Some(chips) = bots.get_mut(&high_to_id) {
                            if high > chips.1 {
                                chips.0 = chips.1;
                                chips.1 = high;
                            } else {
                                chips.0 = high;
                            }
                        }
                    } else if high_to_type == "output" {
                        outputs
                            .entry(high_to_id)
                            .and_modify(|o| *o = high)
                            .or_insert(high);
                    }
                    bots.entry(from_bot_id).and_modify(|c| *c = Chips(0, 0));
                    instruction.executed = true;
                }
            }
        }

        if let Some(_) = instructions.iter().position(|i| i.executed == false) {
            continue;
        } else {
            break;
        }
    }

    println!("Bots:");
    for bot in bots.iter() {
        println!("{:?}", bot);
    }
    println!("Outputs:");
    for output in outputs.iter() {
        println!("{:?}", output);
    }
}
