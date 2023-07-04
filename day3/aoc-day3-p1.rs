use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
fn main() {
    // Open the file
    let mut file = File::open("./raw.txt").expect("Failed to open the file");

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file.");

    let lines: Vec<&str> = contents.lines().collect();

    let mut chars: Vec<char> = Vec::new();

    let mut set_first: HashSet<String> = HashSet::new();

    let mut sum: i32 = 0;
    for line in lines {
        set_first = HashSet::new();
        let half_len = line.len() / 2;
        let p1 = &line[0..half_len];
        let p2 = &line[half_len..];
        println!("Part 1: {}", p1);
        println!("Part 2: {}", p2);

        for ch1 in p1.chars() {
            for ch2 in p2.chars() {
                if ch1 == ch2 {
                    set_first.insert(ch1.to_string());
                }
            }
        }
        let mut get_char: Vec<String> = set_first.into_iter().collect();
        let part = match get_char.remove(0).chars().nth(0) {
            Some(n) => n,
            None => panic!("No value"),
        };
        chars.push(part);
    }
    for char in chars {
        if char >= 'a' && char <= 'z' {
            sum += (char as i32) - 96;
        } else if char >= 'A' && char <= 'Z' {
            sum += (char as i32) - 64 + 26;
        }
    }
    println!("Sum = {:#?}", sum);
}
