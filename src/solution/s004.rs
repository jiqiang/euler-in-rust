pub fn is_palindrome(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    if s.len() == 1 {
        return true;
    }

    if s != s.chars().rev().collect::<String>() {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        struct Test<'a> {
            target: &'a str,
            expected: bool,
        };

        let tests = [
            Test {
                target: "",
                expected: false,
            },
            Test {
                target: "2",
                expected: true,
            },
            Test {
                target: "999999",
                expected: true,
            },
            Test {
                target: "987689",
                expected: false,
            },
            Test {
                target: "123321",
                expected: true,
            },
        ];

        for test in tests.iter() {
            assert_eq!(test.expected, is_palindrome(&test.target));
        }
    }
}
