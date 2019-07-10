//! Problem 28: Number spiral diagonals
//!
//! Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
//!
//! 21 22 23 24 25
//!
//! 20  7  8  9 10
//!
//! 19  6  1  2 11
//!
//! 18  5  4  3 12
//!
//! 17 16 15 14 13
//!
//! It can be verified that the sum of the numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

pub fn number_spiral_diagonals(max_width: u32) -> u32 {
    let mut current_width = 3;
    let mut start = 1;
    let mut sum = 0;
    let mut distance_to_corner: u32;
    let mut right_bottom: u32;
    let mut left_bottom: u32;
    let mut left_top: u32;
    let mut right_top: u32;

    while current_width <= max_width {
        distance_to_corner = current_width - 1;
        right_bottom = start + distance_to_corner;
        left_bottom = right_bottom + distance_to_corner;
        left_top = left_bottom + distance_to_corner;
        right_top = left_top + distance_to_corner;
        start = right_top;
        sum += right_bottom + left_bottom + left_top + right_top;
        current_width += 2;
    }
    sum + 1
}

#[cfg(test)]
mod s028_tests {
    use super::*;

    struct Test {
        width: u32,
        sum: u32,
    }

    #[test]
    fn test_number_spiral_diagonals() {
        let tests = vec![
            Test { width: 5, sum: 101 },
            Test {
                width: 1001,
                sum: 669_171_001,
            },
        ];
        for test in tests.iter() {
            assert_eq!(test.sum, number_spiral_diagonals(test.width));
        }
    }
}
