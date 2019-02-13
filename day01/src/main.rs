use std::collections::HashSet;
use std::io::{self, Stdin};

fn main() -> io::Result<()> {
    let mut sum = 0;
    let mut frequencies = HashSet::new();
    let mut dup_found = false;

    let stdin = io::stdin();
    let input = load_input(&stdin);

    while !dup_found {
        for x in input.iter() {
            sum += x;

            if frequencies.contains(&sum) {
                dup_found = true;
                println!("{}", sum);
                break;
            } else {
                frequencies.insert(sum);
            }
        }
    }

    Ok(())
}

fn load_input(stdin: &Stdin) -> Vec<i32> {
    let mut input = Vec::new();
    let mut buffer = String::new();

    while let Ok(_) = stdin.read_line(&mut buffer) {
        if buffer.starts_with('+') {
            input.push(buffer[1..].trim().parse::<i32>().unwrap());
        } else if buffer.starts_with('-') {
            input.push(buffer[1..].trim().parse::<i32>().unwrap() * -1);
        } else {
            break;
        }

        buffer.clear();
    }

    input
}
