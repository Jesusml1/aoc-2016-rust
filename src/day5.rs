// use hex_literal::hex;
use hex_string::{self, HexString};
use md5::{Digest, Md5};

pub fn execute() {
    let mut num = 0;

    let door_id = "reyedfim".to_string();
    #[allow(unused_assignments)]
    let mut password = String::from("________");
    let mut password_vec = Vec::from(['_', '_', '_', '_', '_', '_', '_', '_']);

    loop {
        let num_str = num.to_string();
        let to_check: String = door_id.clone() + &num_str;
        let mut hasher = Md5::new();
        hasher.update(to_check.clone());
        let result = hasher.finalize();

        let mut hash_vec: Vec<u8> = vec![];
        for r in result {
            hash_vec.push(r);
        }

        let md5_string = HexString::from_bytes(&hash_vec).as_string();
        let (zeros, rest) = md5_string.split_at(5);

        if zeros == "00000" {
            // part 1
            // println!("{}", md5_string);
            // let character = rest.get(0..1).unwrap_or("?");
            // password.push_str(character);
            // if password.len() == 8 {
            //     break;
            // }

            // part 2
            let position = rest.get(0..1).unwrap_or("?");
            let position = match position.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    num += 1;
                    continue
                },
            };
            let character = rest.get(1..2).unwrap_or("?").parse::<char>().unwrap();

            if position < password_vec.len() && password_vec[position] == '_' {
                password_vec[position] = character;
                let mut current_password = String::new();
                for c in password_vec.clone() {
                    current_password.push(c);
                }
                password = current_password;

                println!("n: {}, md5: {}, password: {}", num, md5_string, password);

                if !password.contains("_") {
                    break;
                }
            }
        }

        num += 1;
    }

    println!("password is: {}", password);
}
