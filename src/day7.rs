#[allow(unused_imports)]
use crate::utils;
use std::collections::HashSet;

// tls
#[allow(dead_code)]
fn check_for_abba(sequence: &str) -> bool {
    let mut valid_abba = 0;

    if sequence.len() % 2 != 0 && sequence.len() < 4 {
        return false;
    }

    for (i, _s) in sequence.chars().enumerate() {
        let abba_seq = sequence.get(i..i + 4);
        if let Some(s) = abba_seq {
            let (first, last) = s.split_at(s.len() / 2);
            let mut last_reversed = String::new();

            let mut hashset: HashSet<char> = HashSet::new();

            for c in last.chars().rev() {
                hashset.insert(c);
                last_reversed.push(c);
            }

            if hashset.len() != last.len() {
                continue;
            }

            if first == last_reversed {
                valid_abba += 1;
            }
        }
    }
    if valid_abba < 1 {
        return false;
    }

    return true;
}

#[allow(dead_code)]
fn check_tls_ip_address(sequences: Vec<&str>) -> bool {
    let mut valid_abba = 0;
    for (i, sequence) in sequences.iter().enumerate() {
        if i % 2 == 0 {
            if check_for_abba(sequence) {
                valid_abba += 1;
            }
        } else if i % 2 != 0 {
            if check_for_abba(sequence) {
                return false;
            }
        }
    }
    if valid_abba < 1 {
        return false;
    }
    return true;
}

//ssl
fn flip_seq(sequence: String) -> String {
    let mut seq = sequence.to_string();
    let binding = seq.clone();
    let flip_append = binding.get(1..2).unwrap();
    seq.push_str(flip_append);
    let fliped = seq.get(1..4).unwrap();

    return fliped.to_string();
}

fn check_ssl_ip_address(sequences: Vec<&str>) -> bool {
    let mut superseq: Vec<String> = Vec::new();
    let mut hyperseq: Vec<String> = Vec::new();

    for (i, sequence) in sequences.iter().enumerate() {
        // getting aba's
        if i % 2 == 0 {
            for (i, _s) in sequence.chars().enumerate() {
                let aba_seq = sequence.get(i..i + 3);
                if let Some(s) = aba_seq {
                    superseq.push(s.to_string());
                }
            }
        }
        // getting bab's
        if i % 2 != 0 {
            for (i, _s) in sequence.chars().enumerate() {
                let bab_seq = sequence.get(i..i + 3);
                if let Some(s) = bab_seq {
                    hyperseq.push(s.to_string());
                }
            }
        }
    }

    let mut superset: HashSet<String> = HashSet::new();
    let mut hyperset: HashSet<String> = HashSet::new();

    for entry in superseq {
        let mut entry_vec = Vec::new();
        for c in entry.chars() {
            entry_vec.push(c);
        }

        if entry_vec[0] == entry_vec[2] {
            superset.insert(entry);
        }
    }

    for entry in hyperseq {
        let mut entry_vec = Vec::new();
        for c in entry.chars() {
            entry_vec.push(c);
        }
        if entry_vec[0] == entry_vec[2] {
            let fliped = flip_seq(entry);
            hyperset.insert(fliped);
        }
    }

    if !superset.is_disjoint(&hyperset) {
        return true;
    }

    false
}

pub fn execute() {
    // let input: Vec<String> = Vec::from([
    //     "abba[mnop]qrst".to_string(),
    //     "abcd[bddb]xyyx".to_string(),
    //     "aaaa[qwer]tyui".to_string(),
    //     "ioxxoj[asdfgh]zxcvbn".to_string(),
    // ]);

    // let input: Vec<String> = Vec::from([
    //     "aba[bab]xyz".to_string(),
    //     "xyx[xyx]xyx".to_string(),
    //     "aaa[kek]eke".to_string(),
    //     "zazbz[bzb]cdb".to_string(),
    // ]);

    // for entry in input {
    //     let ip = entry.clone();
    //     let v: Vec<&str> = ip.split(['[', ']']).collect();
    //     println!("{:?}, valid: {}", v.clone(), check_ssl_ip_address(v));
    // }

    let mut valid_tls_ips = 0;
    let mut valid_ssl_ips = 0;
    let lines = utils::read_lines("input/day7_input.txt").unwrap();
    for line in lines {
        if let Ok(line) = line {
            let ip = line.clone();
            let v: Vec<&str> = ip.split(['[', ']']).collect();
            if check_tls_ip_address(v.clone()) {
                valid_tls_ips += 1;
            }
            if check_ssl_ip_address(v) {
                valid_ssl_ips += 1;
            }
        }
    }

    println!("ip's that support tls: {}", valid_tls_ips);
    println!("ip's that support ssl: {}", valid_ssl_ips);
}
