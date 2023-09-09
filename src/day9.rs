use std::fs;

fn decompress(expression: String) -> usize {
    let v: Vec<char> = expression.chars().collect();

    let mut result: usize = 0;

    let mut capture_ins: bool = false;
    let mut capture_seq: bool = false;

    let mut instruction: String = String::new();
    let mut sequence = String::new();

    let mut nc: usize = 0;
    let mut nr: usize = 0;

    if !expression.contains("(") && !expression.contains(")") {
        return expression.len()
    }

    for c in v {
        if c == '(' && !capture_ins && !capture_seq {
            capture_ins = true;
        } else if capture_ins && c != '(' && c != ')' && !capture_seq {
            instruction.push(c);
        } else if c == ')' && capture_ins && !capture_seq {
            capture_ins = false;
            let parsed_ins = parse_ins(&instruction);
            nc = parsed_ins.0;
            nr = parsed_ins.1;
            capture_seq = true;
        } else if capture_seq && nc > 0 {
            sequence.push(c);
            nc -= 1;
            if nc == 0 {
                result += decompress(sequence.repeat(nr));
                // println!("{}", result);
                capture_seq = false;
                capture_ins = false;
                instruction.clear();
                sequence.clear();
                nr = 0;
            }
        } else {
            result += 1;
        }
    }

    result
}

fn parse_ins(ins: &String) -> (usize, usize) {
    let (nc, nr) = ins.split_once("x").unwrap();
    let nc = nc.parse::<usize>().unwrap();
    let nr = nr.parse::<usize>().unwrap();

    (nc, nr)
}

pub fn execute() {
    // let examples: Vec<String> = vec![
    //     "ADVENT".to_string(),
    //     "A(1x5)BC".to_string(),
    //     "(3x3)XYZ".to_string(),
    //     "A(2x2)BCD(2x2)EFG".to_string(),
    //     "(6x1)(1x3)A(6x1)(1x3)A".to_string(),
    //     "X(8x2)(3x3)ABCYX(8x2)(3x3)ABCY".to_string(),
    //     "ADVENT\nA(1x5)BC".to_string(),
    // ];

    // for expression in examples {
    //     let content = decompress(expression.clone());
    //     println!("{} -> {}, len: {}", expression, content, content.len());
    // }

    let sample =
        fs::read_to_string("input/day9_input.txt").expect("Should have been able to read the file");

    let content = decompress(sample);
    println!("{}", content);

}
