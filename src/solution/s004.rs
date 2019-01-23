pub fn is_palindrome_1(s: &str) -> bool {
    // A String type can be magically turned into a &str type using the Deref trait and type coercion.
    if s != s.chars().rev().collect::<String>() {
        return false;
    }
    true
}

pub fn is_palindrome_2(s: &str) -> bool {
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test<'a> {
        target: &'a str,
        expected: bool,
    }

    fn get_tests<'a>() -> Vec<Test<'a>> {
        vec![
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
            Test {
                target: "1234321",
                expected: true,
            },
        ]
    }

    #[test]
    fn test_is_palindrome_1() {
        for test in get_tests().iter() {
            assert_eq!(test.expected, is_palindrome_1(test.target));
        }
    }

    #[test]
    fn test_is_palindrome_2() {
        for test in get_tests().iter() {
            assert_eq!(test.expected, is_palindrome_2(test.target));
        }
    }
}
