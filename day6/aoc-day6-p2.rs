use std::fs::File;
use std::io::Read;

fn check_rep(str: &str) -> bool {
    for ch in str.chars() {
        let mut frq: i32 = 0;
        for ltr in str.chars() {
            if ch == ltr {
                frq += 1;
            }
        }
        if frq > 1 {
            return false;
        }
    }
    true
}

fn main() {
    let mut file = File::open("./raw.txt").expect("Failed to open the file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file");

    for i in 0..contents.len() - 14 {
        let slce = &contents[i..i + 14];
        if check_rep(slce) {
            println!("{}", slce);
            println!("{}", i + 14);
            break;
        };
        // println!("{}", slce);
    }
    // println!("{:#?}", contents);
}
