//! Two sum problem solution
//!
//! Given an array of integers nums and an integer target, return indices of the 
//! two numbers such that they add up to target.

use std::collections::HashMap;

/// Finds two indices in the array that sum to the target
///
/// # Arguments
///
/// * `nums` - A slice of integers
/// * `target` - The target sum
///
/// # Returns
///
/// `Some((i, j))` with indices of the two numbers that sum to target,
/// or `None` if no solution exists
///
/// # Example
///
/// ```
/// use problem_solving::arrays::two_sum::two_sum;
///
/// let nums = vec![2, 7, 11, 15];
/// assert_eq!(two_sum(&nums, 9), Some((0, 1)));
/// ```
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        
        map.insert(num, i);
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
        assert_eq!(two_sum(&[1, 2, 3, 4], 10), None);
    }
}

