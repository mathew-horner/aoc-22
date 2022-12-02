use std::collections::BinaryHeap;
use std::fs;

pub fn solution() {
    let input = fs::read_to_string("./data/input-day1.txt").unwrap();
    let blocks = input.split("\n\n").map(str::trim);
    let mut heap: BinaryHeap<u32> = blocks
        .map(|block| {
            block
                .split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    let top = heap.pop().unwrap();
    println!("Answer #1: {}", top);

    let top3 = top + heap.pop().unwrap() + heap.pop().unwrap();
    println!("Answer #2: {}", top3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn day1() {
        solution();
    }
}
