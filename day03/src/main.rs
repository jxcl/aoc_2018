use std::cmp;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn max(&self) -> usize {
        cmp::max(self.x as usize, self.y as usize)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(Debug)]
struct Claim {
    top_left: Point,
    dim: Point,
}

#[derive(Debug)]
struct Fabric {
    plots: Vec<Vec<u8>>,
    claims: Vec<Claim>,
    size: usize,
}

impl Fabric {
    fn new(size: usize) -> Fabric {
        let claims = Vec::new();
        let mut plots = Vec::with_capacity(size);
        for _x in 0..size {
            let mut v = Vec::with_capacity(size);
            for _y in 0..size {
                v.push(0);
            }
            plots.push(v);
        }

        Fabric {plots: plots, size: size, claims: claims}
    }

    fn grow(&mut self, target_size: usize) {
        let diff = target_size - self.size;

        for v in self.plots.iter_mut() {
            println!("plots");
            for _ in 0..diff {
                println!("push");
                v.push(0);
            }
        }

        for _ in 0..diff {
            let mut new_vec = Vec::with_capacity(target_size);
            for _ in 0..target_size {
                new_vec.push(0);
            }

            self.plots.push(new_vec);
        }

        self.size = target_size;
    }

    fn claim(&mut self, top_left: Point, dim: Point) {
        let bottom_right = top_left + dim;
        let required_size = bottom_right.max();

        if self.size < required_size {
            self.grow(required_size);
        }

        for x in self.plots.iter_mut().skip(top_left.x as usize).take((bottom_right.x - top_left.x) as usize) {
            for y in x.iter_mut().skip(top_left.y as usize).take((bottom_right.y - top_left.y) as usize) {
                *y += 1
            }
        }

        self.claims.push(Claim {top_left, dim});
    }

    fn claim_overlapped(&self, claim: &Claim) -> bool {
        for x in self.plots.iter().skip(claim.top_left.x as usize).take(claim.dim.x as usize) {
            for y in x.iter().skip(claim.top_left.y as usize).take(claim.dim.y as usize) {
                if *y > 1 {
                    return true;
                }
            }
        }

        false
    }

    fn count_overlapping_plots(&self) -> usize {
        let mut overlaps = 0;

        for x in self.plots.iter() {
            for y in x.iter() {
                if *y > 1 {
                    overlaps += 1;
                }
            }
        }

        overlaps
    }
}

fn parse_tuple<T: FromStr>(input: &str, sep: char) -> Option<(T, T)> {
    match input.find(sep) {
        None => None,
        Some(i) => {
            match (input[..i].parse(), input[i+1..].parse()) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn make_claim(tup: (i32, i32)) -> Point {
    Point { x: tup.0, y: tup.1 }
}

fn parse_claim(string_claim: &str) -> (Point, Point) {
    let components = string_claim.split(" ").skip(2).collect::<Vec<&str>>();

    let top_left = parse_tuple::<i32>(components[0].trim_matches(':'), ',').unwrap();
    let dim = parse_tuple::<i32>(components[1], 'x').unwrap();

    let top_left = make_claim(top_left);
    let dim = make_claim(dim);

    (top_left, dim)
}

fn part1(input: &str) {
    let mut fab = Fabric::new(1000);

    for line in input.lines() {
        let (top_left, dim) = parse_claim(line.trim());
        fab.claim(top_left, dim);
    }

    println!("{}", fab.count_overlapping_plots());

    for (i, claim) in fab.claims.iter().enumerate() {
        if !fab.claim_overlapped(claim) {
            println!("{}", i);
            break;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part1(&input);
}
