/*
--- Day 4: Camp Cleanup ---
Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
For the first few pairs, this list means:

Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
The Elves in the second pair were each assigned two sections.
The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8
Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?
*/
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



pub fn first_challenge() {
    let mut total: u16 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(line_text) = line {
                if does_one_range_contains_other(&line_text) == true {
                    total+=1;
                };
            }
        }
    }
    println!("Total items: {}",total);
}

fn read_lines<P>(filename: P) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn does_one_range_contains_other(text: &String) -> bool {    
    let (first_part, second_part) = split_to_parts(text);
    let (first_start, first_end) = split_to_start_and_end(first_part);
    let (second_start, second_end) = split_to_start_and_end(second_part);
    return first_contains_second(&first_start, &first_end, &second_start, &second_end) || second_contains_first(&first_start, &first_end, &second_start, &second_end);
}

fn split_to_parts(text: &String) -> (&str, &str) {
    let tokens: Vec<&str> = text.split(",").collect();
    let first_part = tokens[0];
    let second_part = tokens[1];
    return (first_part, second_part);
}

fn split_to_start_and_end(text: &str) -> (u16, u16) {
    let tokens: Vec<&str> = text.split("-").collect();
    let start: u16 = tokens[0].trim().parse().unwrap();
    let end: u16 = tokens[1].trim().parse().unwrap();
    return (start, end);
}

fn first_contains_second(first_start: &u16, first_end: &u16, second_start: &u16, second_end: &u16) -> bool {
    return first_start <= second_start && first_end >= second_end;
}

fn second_contains_first(first_start: &u16, first_end: &u16, second_start: &u16, second_end: &u16) -> bool {
    return second_start <= first_start && second_end >= first_end;
}
