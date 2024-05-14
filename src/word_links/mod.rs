fn probability_that_arrangement_is_crossless(n: u32) -> f64 {
    (2_f64.powf((n - 2) as f64) * 5.0) / (factorial(n) as f64)
}
fn factorial(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::word_links::factorial;

    use super::probability_that_arrangement_is_crossless;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 5 * 4 * 3 * 2 * 1)
    }

    #[test]
    fn problem_d() {
        for i in 2.. {
            // I assume it is a percentage.
            let prob = 100.0 * probability_that_arrangement_is_crossless(i);
            dbg!(i, prob);
            if prob < 0.01 {
                eprintln!("found num: {i} ({prob}%)");
                assert_eq!(i, 11);
                return;
            }
        }
    }
}
