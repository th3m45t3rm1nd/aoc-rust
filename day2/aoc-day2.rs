use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    // Open the file
    let mut file = File::open("../../raw.txt").expect("Failed to Open the file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    //Part 1
    /* let strategy1 = vec![
        ("A X", 4),
        ("B X", 1),
        ("C X", 7),
        ("A Y", 8),
        ("B Y", 5),
        ("C Y", 2),
        ("A Z", 3),
        ("B Z", 9),
        ("C Z", 6),
    ];

    let strat_guide1: HashMap<&str, i32> = strategy1.into_iter().collect();

    let mut score1: i32 = 0;

    for line in lines {
        let value = strat_guide1.get(line);
        match value {
            Some(n) => score1 += n,
            None => score1 += 0,
        }
    } */

    //Part 2
    let strategy2 = vec![
        ("A X", 3),
        ("B X", 1),
        ("C X", 2),
        ("A Y", 4),
        ("B Y", 5),
        ("C Y", 6),
        ("A Z", 8),
        ("B Z", 9),
        ("C Z", 7),
    ];

    let strat_guide2: HashMap<&str, i32> = strategy2.into_iter().collect();

    let mut score2: i32 = 0;

    for line in lines {
        let value = strat_guide2.get(line);
        match value {
            Some(n) => score2 += n,
            None => score2 += 0,
        }
    }
    println!("Your Score: {}", score2);
}
