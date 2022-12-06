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
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(count)
        .enumerate()
        .find(|(_, window)| HashSet::<_>::from_iter(window.iter()).len() == count)
        .map(|(i, _)| i + count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn day6() {
        solution();
    }
}
