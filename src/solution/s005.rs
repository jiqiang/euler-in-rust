pub fn is_prime(num: u32) -> bool {
    num != 0 && (2..num).find(|n| num % n == 0) == None
}

pub fn get_prime_factors(mut num: u32) -> Vec<u32> {
    let mut lcm: Vec<u32> = Vec::new();
    let mut n = 2;
    loop {
        if n > num {
            break;
        }

        if is_prime(n) && num % n == 0 {
            lcm.push(n);
            num /= n;
        } else {
            n += 1;
        }
    }
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let tests = [
            (false, 0),
            (true, 1),
            (true, 2),
            (true, 3),
            (false, 4),
        ];
        for test in tests.iter() {
            assert_eq!(test.0, is_prime(test.1));
        }
    }

    #[test]
    fn test_get_prime_factors() {
        let tests = [
            (vec![1], 1),
            (vec![2], 2),
            (vec![3], 3),
            (vec![2, 2], 4),
            (vec![5], 5),
            (vec![2, 3], 6),
            (vec![2, 2, 5, 5], 100),
        ];
        for test in tests.iter() {
            assert_eq!(test.0, get_prime_factors(test.1));
        }
    }
}
