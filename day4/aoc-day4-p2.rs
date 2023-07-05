use std::fs::File;
use std::io::Read;

fn extractor(line: &str) -> Vec<(i32, i32)> {
    let parts: Vec<&str> = line.split(',').collect();
    let mut from_and_to: Vec<(i32, i32)> = Vec::new();
    for part in parts {
        let nums: Vec<&str> = part.split('-').collect();
        let mut temp: Vec<i32> = Vec::new();
        for num in nums {
            let parsed_str: Result<i32, _> = num.parse();
            match parsed_str {
                Ok(n) => temp.push(n),
                Err(_) => println!("Error parsing the value"),
            }
        }
        from_and_to.push((temp[0], temp[1]));
    }
    from_and_to
}

fn range_in(tuples: Vec<(i32, i32)>) -> bool {
    if (tuples[1].0..=tuples[1].1).contains(&tuples[0].0)
        || (tuples[1].0..=tuples[1].1).contains(&tuples[0].1)
    {
        true
    } else if (tuples[0].0..=tuples[0].1).contains(&tuples[1].0)
        || (tuples[0].0..=tuples[0].1).contains(&tuples[1].1)
    {
        true
    } else {
        false
    }
}

fn main() {
    // Open the file
    let mut file = File::open("./raw.txt").expect("Failed to open the File.");

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the File");

    let lines: Vec<&str> = contents.lines().collect();

    let mut sum: i32 = 0;

    for line in lines {
        if range_in(extractor(line)) {
            sum += 1;
        }
    }
    println!("Sum = {:#?}", sum);
}
