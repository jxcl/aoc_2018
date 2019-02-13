use std::cmp;
use std::io::{self, Read};

fn polymers_reduce(a: &u8, b: &u8) -> bool {
    // 'A' and 'a' will compare as unequal `u8`s but
    // will compare as equal when case is ignored.

    (*a != *b && a.eq_ignore_ascii_case(b))
}

fn reduce_forward(input: &[u8]) -> String {
    let mut processed: Vec<u8> = Vec::new();

    for ch in input {
        if (*ch as char).is_whitespace() {
            break;
        }

        if processed.len() == 0 {
            processed.push(*ch);
        } else {
            if polymers_reduce(&processed[processed.len() - 1], ch) {
                processed.pop();
            } else {
                processed.push(*ch);
            }
        }
    }

    String::from_utf8(processed).unwrap()
}

fn part1(input: &[u8]) {
    println!("{}", reduce_forward(input).len());
}

fn part2(input: &[u8]) {
    let mut min_length = 11364; // Length of answer from part 1

    for x in b'a'..b'z' {
        let filtered: Vec<u8> = input.into_iter().filter(|el| !el.eq_ignore_ascii_case(&x)).cloned().collect();
        let reduced_len = reduce_forward(&filtered).len();

        min_length = cmp::min(reduced_len, min_length);
    }

    println!("{}", min_length);
}

fn main() {
    let mut input: Vec<u8> = Vec::new();

    io::stdin().read_to_end(&mut input).unwrap();

    part1(&input);
    part2(&input);
}
