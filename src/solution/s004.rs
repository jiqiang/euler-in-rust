pub fn is_palindrome_1(s: &str) -> bool {
    // A String type can be magically turned into a &str type using the Deref trait and type coercion.
    s == s.chars().rev().collect::<String>()
}

pub fn is_palindrome_2(s: &str) -> bool {
    let half = s.len() / 2;
    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

pub fn largest_palindrome_product(lower_boundary: u32, upper_boundary: u32) -> Option<u32> {
    (lower_boundary.pow(2)..=upper_boundary.pow(2))
        .rev()
        .find(|n| {
            is_palindrome_2(&n.to_string())
                && (lower_boundary..=upper_boundary)
                    .find(|n1| n % n1 == 0 && n / n1 >= lower_boundary && n / n1 <= upper_boundary)
                    != None
        })
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
                target: "",
                expected: true,
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

    #[test]
    fn test_largest_palindrome_product() {
        assert_eq!(Some(9009), largest_palindrome_product(10, 99));
        assert_eq!(Some(906_609), largest_palindrome_product(100, 999));
    }
}
