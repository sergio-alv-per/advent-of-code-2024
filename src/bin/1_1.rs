use std::io;

fn main() {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

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
                col2.push(int2);
            }
        }
    }

    col1.sort();
    col2.sort();

    let result: i32 = col1
        .iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{result}");
}
