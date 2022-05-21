fn enumerate_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];

    for i in 2..n {
        if is_prime[i] {
            let k_max = n / i;
            for j in 2..=k_max {
                is_prime[i * j] = false;
            }
        }
    }

    let mut result = Vec::new();
    for i in 2..n {
        if is_prime[i] {
            result.push(i);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(2, vec![])]
    #[case(8, vec![2, 3, 5, 7])]
    #[case(14, vec![2, 3, 5, 7, 11, 13])]
    #[case(21, vec![2, 3, 5, 7, 11, 13, 17, 19])]
    fn test_enumerate_primes(#[case] n: usize, #[case] expected: Vec<usize>) {
        assert_eq!(enumerate_primes(n), expected);
    }
}
