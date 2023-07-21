use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

fn get_pos(line: &str) -> Vec<usize> {
    let mut arr: Vec<usize> = Vec::new();
    let mut pos: i32 = 0;

    for ch in line.chars() {
        if ch.is_alphabetic() {
            arr.push(pos.try_into().unwrap());
        }
        pos += 1;
    }

    arr
}

fn get_chars(lines: &Vec<&str>, pos: &Vec<usize>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut chars: Vec<char> = Vec::new();
    for p in pos {
        for line in lines {
            if let Some(c) = line.chars().nth(*p) {
                if c.is_whitespace() {
                    continue;
                }
                chars.push(c);
            }
        }
        stacks.push(chars.clone());
        chars = Vec::new();
    }
    stacks
}

fn get_moves(proc: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut to_return: Vec<Vec<i32>> = Vec::new();
    for p in proc {
        let mut some_vec: Vec<i32> = Vec::new();
        let words: Vec<&str> = p.split(' ').collect();
        for word in words {
            if let Ok(num) = word.parse() {
                some_vec.push(num);
            }
        }
        to_return.push(some_vec);
    }
    to_return
}

fn change_stack(
    stack_from: &Vec<char>,
    stack_to: &Vec<char>,
    how_many: i32,
) -> (Vec<char>, Vec<char>) {
    let mut new_stack_from: Vec<char> = stack_from.to_vec();
    let mut new_stack_to: Vec<char> = stack_to.to_vec();
    let mut temp_vec: Vec<char> = Vec::new();
    for _i in 0..how_many {
        let char = new_stack_from.remove(0);
        temp_vec.push(char);
    }
    new_stack_to.splice(0..0, temp_vec.clone());
    (new_stack_from.to_vec(), new_stack_to.to_vec())
}

fn main() {
    // Open the File
    let mut file = File::open("./raw.txt").expect("Failed to Open the File");

    // Read the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file.");

    let lines: Vec<&str> = contents.lines().collect();
    let pos = get_pos(lines[7]);

    let crates: Vec<&str> = lines[0..8].to_vec();
    let rearr_proc: Vec<&str> = lines[10..].to_vec();

    let mut stacks: Vec<Vec<char>> = get_chars(&crates, &pos);
    let moves: Vec<Vec<i32>> = get_moves(&rearr_proc);

    for m in moves {
        (
            stacks[(m[1] as usize).wrapping_sub(1)],
            stacks[(m[2] as usize).wrapping_sub(1)],
        ) = change_stack(
            &stacks[(m[1] as usize).wrapping_sub(1)],
            &stacks[(m[2] as usize).wrapping_sub(1)],
            m[0],
        );
    }
    println!("{:#?}", stacks);
}
