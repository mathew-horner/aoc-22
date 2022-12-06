#![allow(unused)]
use std::collections::{HashSet, VecDeque};
use std::ops::Deref;

pub fn solution() {
    let input = std::fs::read_to_string("./data/input-day6.txt").unwrap();
    let mut start_of_packet = find_first_marker(&input, 4).unwrap();
    let mut start_of_message = find_first_marker(&input, 14).unwrap();
    println!("Answer #1: {}", start_of_packet);
    println!("Answer #2: {}", start_of_message);
}

fn find_first_marker(input: &str, count: usize) -> Option<usize> {
    let mut iter = input.chars().enumerate();
    let mut queue = VecDeque::new();
    let mut i = 0;

    for _ in 0..(count - 1) {
        queue.push_back(iter.next().unwrap().1);
    }

    while let Some((i, next)) = iter.next() {
        queue.push_back(next);
        let unique: HashSet<char> = HashSet::from_iter(queue.iter().map(|&c| c));

        if unique.len() == count {
            return Some(i + 1);
        }

        _ = queue.pop_front();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn day6() {
        solution();
    }
}
