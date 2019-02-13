use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut two = false;
        let mut three = false;

        let mut letter_counts = HashMap::new();

        for ch in line.trim().chars() {
            let counter = letter_counts.entry(ch).or_insert(0);
            *counter += 1;
        }

        for (key, value) in letter_counts.iter() {
            if *value == 2 {
                two = true;
            } else if *value == 3 {
                three = true;
            }
        }

        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }

    println!("{}", twos * threes);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    for (i, line1) in input.lines().enumerate() {
        for line2 in input.lines().skip(i) {
            let distance = string_distance(line1.trim(), line2.trim());

            if distance == 1 {
                println!("{}\n{}", line1, line2);
            }
        }
    }

    Ok(())
}

fn string_distance(str1: &str, str2: &str) -> usize {
    let mut distance = 0;

    let char_comparison_iter = str1.chars().zip(str2.chars());

    for (ch1, ch2) in char_comparison_iter {
        if ch1 != ch2 {
            distance += 1;
        }
    }

    distance
}
