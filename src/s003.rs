//! Problem 3: Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143?

pub fn largest_prime_factor(mut num: usize) -> usize {
    let mut index = 2;
    while index < num {
        if num % index == 0 {
            num /= index;
            index -= 1;
        }
        index += 1;
    }
    num
}

#[cfg(test)]
mod s003_tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(29, largest_prime_factor(13195));
        assert_eq!(6857, largest_prime_factor(600_851_475_143));
    }
}
