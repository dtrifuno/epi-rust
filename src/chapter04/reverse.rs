pub fn reverse(n: i32) -> i64 {
    let mut result: i64 = 0;
    let negative = n < 0;
    let mut remainder: i64 = n.abs() as i64;

    while remainder > 0 {
        result *= 10;
        result += remainder % 10;
        remainder /= 10;
    }

    match negative {
        true => -result,
        false => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(54033200, 233045)]
    #[case(-234131500, -5131432)]
    fn reverse_ints(#[case] input: i32, #[case] expected: i64) {
        assert_eq!(expected, reverse(input));
    }
}
