use std::collections::HashMap;
use std::io;

fn main() {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();
    loop {
        let mut numbers = String::new();
        let res = io::stdin().read_line(&mut numbers);
        match res {
            Err(_) | Ok(0) => break,
            Ok(_) => {
                let (str1, str2) = numbers.split_once("   ").unwrap();
                let str2 = str2.strip_suffix("\n").unwrap();
                let int1: i32 = str1.parse().unwrap();
                let int2: i32 = str2.parse().unwrap();
                col1.push(int1);

                let v = col2.get(&int2);

                match v {
                    Some(freq) => col2.insert(int2, freq + 1),
                    None => col2.insert(int2, 1),
                };
            }
        }
    }

    let mut result = 0;

    for v1 in col1 {
        let freq = col2.get(&v1);
        result += match freq {
            Some(f) => v1 * f,
            None => 0,
        };
    }
    println!("{result}");
}
