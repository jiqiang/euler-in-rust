pub fn run(max_num: usize) -> usize {
    let mut pair: (usize, usize) = (1, 2);
    let mut sum: usize = 0;
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

    #[test]
    fn p002() {
        assert_eq!(0, run(0));
        assert_eq!(2, run(2));
        assert_eq!(2, run(3));
        assert_eq!(2, run(4));
        assert_eq!(2, run(5));
        assert_eq!(44, run(89));
    }
}
