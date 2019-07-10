pub fn smallest_multiple(max: usize) -> usize {
    let mut is_lcm: bool;
    let mut lcm: usize = max;
    let mut i: usize = 2;
    loop {
        is_lcm = true;

        for n in 2..max {
            if lcm % n != 0 {
                is_lcm = false;
                break;
            }
        }

        if is_lcm {
            break;
        }

        lcm = max * i;
        i += 1;
    }
    lcm
}

#[cfg(test)]
mod s005_tests {
    use super::*;

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(2520, smallest_multiple(10));
        assert_eq!(232_792_560, smallest_multiple(20));
    }
}
