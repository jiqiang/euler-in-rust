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
