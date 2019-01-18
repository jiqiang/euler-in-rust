pub fn even_fibonacci_numbers_1(max_num: u32) -> u32 {
    let mut pair: (u32, u32) = (1, 2);
    let mut sum: u32 = 0;
    loop {
        if pair.0 > max_num {
            break;
        }

        if pair.0 % 2 == 0 {
            sum += pair.0;
        }

        pair = (pair.1, pair.0 + pair.1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        result: u32,
        max_num: u32,
    }

    #[test]
    fn p002() {
        let tests = [
            Test {
                result: 0,
                max_num: 0,
            },
            Test {
                result: 2,
                max_num: 2,
            },
            Test {
                result: 2,
                max_num: 3,
            },
            Test {
                result: 2,
                max_num: 4,
            },
            Test {
                result: 2,
                max_num: 5,
            },
            Test {
                result: 44,
                max_num: 89,
            },
        ];

        for test in tests.iter() {
            assert_eq!(test.result, even_fibonacci_numbers_1(test.max_num));
        }
    }
}
