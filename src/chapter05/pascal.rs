fn pascal_triangle(n: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::with_capacity(n);

    for i in 0..n {
        let mut current_row = Vec::with_capacity(i + 1);
        for j in 0..=i {
            if j > 0 && j < i {
                current_row.push(result[i - 1][j - 1] + result[i - 1][j]);
            } else {
                current_row.push(1);
            }
        }
        result.push(current_row);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, vec![])]
    #[case(1, vec![vec![1]])]
    #[case(2, vec![vec![1], vec![1, 1]])]
    #[case(3, vec![vec![1], vec![1, 1], vec![1, 2, 1]])]
    fn test_pascal_triangle(#[case] size: usize, #[case] expected: Vec<Vec<usize>>) {
        assert_eq!(expected, pascal_triangle(size));
    }
}
