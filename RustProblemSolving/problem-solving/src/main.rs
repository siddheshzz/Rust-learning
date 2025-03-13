//! Example usage of problem-solving algorithms

use problem_solving::{anagram_for, contains_duplicate, two_sum};

fn main() {
    println!("Problem Solving Algorithms");
    
    // Two Sum example
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    match two_sum(&nums, target) {
        Some((i, j)) => println!("Two Sum: Found indices {} and {} that sum to {}", i, j, target),
        None => println!("Two Sum: No solution found"),
    }
    
    // Contains Duplicate example
    let nums = vec![1, 2, 3, 1];
    let result = contains_duplicate(&nums);
    println!("Contains Duplicate: {}", result);
    
    // Anagram example
    let word = "orchestra";
    let candidates = &["cashregister", "Carthorse", "radishes"];
    let anagrams = anagram_for(word, candidates);
    println!("Anagrams for '{}': {:?}", word, anagrams);
}
