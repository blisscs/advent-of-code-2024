use regex::Regex;
use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("../input.txt").expect("Should be able to read from input.txt");

    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    let pattern = Regex::new(r"(?<left>\d+)\s+(?<right>\d+)").unwrap();

    input.as_str().split('\n').for_each(|line| {
        let captures = pattern.captures(line);
        match captures {
            Some(caps) => {
                let left: u32 = caps
                    .name("left")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be able to parse the number");
                let right: u32 = caps
                    .name("right")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("should be able to parse the number");

                list1.push(left);
                list2.push(right);
            }
            None => {}
        }
    });

    list1.sort();
    list2.sort();

    let pairs: Vec<(&u32, &u32)> = list1.iter().zip(list2.iter()).collect();

    let mut total_distance: u32 = 0;

    for pair in pairs.iter() {
        let (first, second) = pair;
        total_distance = first.abs_diff(**second) + total_distance;
    }

    println!("total_distance = {}", total_distance);
}
