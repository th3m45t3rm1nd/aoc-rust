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

    let mut sum: i32 = 0;
    for i in (0..lines.len() - 2).step_by(3) {
        let mut set_first: HashSet<String> = HashSet::new();
        for ch1 in lines[i].chars() {
            for ch2 in lines[i + 1].chars() {
                if ch1 == ch2 {
                    for ch3 in lines[i + 2].chars() {
                        if ch1 == ch3 {
                            set_first.insert(ch1.to_string());
                        }
                    }
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
