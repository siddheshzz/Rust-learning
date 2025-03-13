//! Contains duplicate problem solution
//!
//! Given an integer array nums, return true if any value appears
//! at least twice in the array, and return false if every element is distinct.

use std::collections::HashSet;

/// Determines if the array contains any duplicate elements
///
/// # Arguments
///
/// * `nums` - A slice of integers
///
/// # Returns
///
/// `true` if the array contains duplicates, `false` otherwise
///
/// # Example
///
/// ```
/// use problem_solving::arrays::contains_duplicate::contains_duplicate;
///
/// let nums = vec![1, 2, 3, 1];
/// assert_eq!(contains_duplicate(&nums), true);
///
/// let nums = vec![1, 2, 3, 4];
/// assert_eq!(contains_duplicate(&nums), false);
/// ```
pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();
    
    for &num in nums {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(&[1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(&[1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}

