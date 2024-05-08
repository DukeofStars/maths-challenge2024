pub fn get_count(z: u32) -> u32 {
    if z == 1 {
        return 2;
    } else if z == 2 {
        return 5;
    }

    let y = z - 1;
    let x = z - 2;

    let x_count = get_count(x);
    let y_count = get_count(y);

    x_count + y_count * 2
}

#[cfg(test)]
mod tests {
    use super::get_count;

    #[test]
    fn first_three_numbers_test() {
        assert_eq!(get_count(1), 2);
        assert_eq!(get_count(2), 5);
        assert_eq!(get_count(3), 12);
    }

    #[test]
    fn problem_b() {
        assert_eq!(get_count(10), 5741);
    }
}
