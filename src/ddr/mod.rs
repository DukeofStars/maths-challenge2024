// Get the DDR of a number.
pub fn ddr_of(x: u32) -> u8 {
    // Convert it to a string and get the digits.
    let s = x.to_string();
    let chars = s.chars().collect::<Vec<_>>();
    // If it only has one digit, just return it.
    if chars.len() == 1 {
        return chars.first().unwrap().to_digit(10).unwrap() as u8;
    }

    let new_x =
    // Iterate of the digits in pairs of 2.
    // e.g. [1, 2, 3, 4, 5] -> (1, 2), (2, 3), (3, 4)
    chars
        .as_slice()
        .windows(2)
        // Get the difference of the two digits.
        .map(|window| {
            let &[a, b] = window else { panic!() };
            let a = a.to_digit(10).unwrap();
            let b = b.to_digit(10).unwrap();
            a.abs_diff(b)
        })
        // Convert the numbers to strings
        .map(|n| n.to_string())
        // Then collect into one string
        .collect::<String>()
        // Convert back into a number
        .parse()
        .unwrap();

    // Repeat the whole process on new number
    ddr_of(new_x)
}

#[cfg(test)]
mod tests {
    use crate::ddr::ddr_of;

    #[test]
    fn problem_a() {
        // These are the answers.
        let answer_set = vec![109, 119, 199, 800, 880, 890, 901, 911, 991];
        let generated =
        // Iterate over all 3 digit numbers.
        (100..=999)
            .into_iter()
            // Get only those with a DDR of 8.
            .filter(|x| ddr_of(*x) == 8)
            .collect::<Vec<_>>();
        assert_eq!(answer_set, generated);
    }

    #[test]
    fn problem_b() {
        let answer = 126;

        let result = (100..=999).into_iter().filter(|x| ddr_of(*x) == 0).count();
        assert_eq!(result, answer);
    }

    #[test]
    // We expect this to panic because there should be no pairs of consecutive 3 digit numbers with a DDR of 0.
    #[should_panic]
    fn problem_c() {
        let mut prev_is_ddr_0 = false;
        // Iterate over all 3 digit numbers.
        for i in 100..=999 {
            // Get the DDR of the number.
            let ddr = ddr_of(i);
            if ddr == 0 {
                // If the previous number also had DDR of 0, then a pair has been found.
                if prev_is_ddr_0 {
                    return;
                } else {
                    prev_is_ddr_0 = true;
                }
            } else {
                prev_is_ddr_0 = false;
            }
        }
        // If none have been found, panic (failed the test).
        panic!()
    }

    #[test]
    fn problem_d() {
        // These should be the smallest consecutive numbers with a DDR of 0.
        let answer = (1379, 1380);

        let mut result = (0, 0);
        let mut prev_is_ddr_0 = false;
        // Iterate over all numbers with at least two digits.
        for i in 10.. {
            // Get the DDR of the number.
            let ddr = ddr_of(i);
            if ddr == 0 {
                // If the previous number also had DDR of 0, then a pair has been found.
                if prev_is_ddr_0 {
                    result = (i - 1, i);
                    break;
                } else {
                    prev_is_ddr_0 = true;
                }
            } else {
                prev_is_ddr_0 = false;
            }
        }

        assert_eq!(result, answer);
    }
}
