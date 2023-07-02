use std::fs::File;
use std::io::Read;

fn main() {
    // Open the file
    let mut file =
        File::open("./raw.txt").expect("Failed to Open the File");

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file");

    // Extracting numbers in a vector
    let lines: Vec<&str> = contents.lines().collect();

    let mut numbers: Vec<i32> = Vec::new();

    let mut sum: i32 = 0;

    for line in lines {
        if line == "" {
            numbers.push(sum);
            sum = 0;
        } else {
            let num: Result<i32, _> = line.parse();

            match num {
                Ok(n) => {
                    sum += n;
                }
                Err(e) => {
                    println!("Failed. {}", e);
                }
            }
        }
    }
    numbers.sort();
    let sorted_len = numbers.len();

    let max_three: i32 =
        numbers[sorted_len - 1] + numbers[sorted_len - 2] + numbers[sorted_len - 3];

    println!("Max Three Sum is: {}", max_three)
}
