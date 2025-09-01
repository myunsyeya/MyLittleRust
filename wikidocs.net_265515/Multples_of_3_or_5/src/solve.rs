#[test]
fn test() {
    assert_eq!(23, get_sum(10));
    assert_eq!(233168, get_sum(1000));
    assert_eq!(233333333166666668, get_sum(1_000_000_000));
}

#[cfg(test)]
fn get_sum(bound: u64) -> u64 {
    sum_under(3, bound - 1) + sum_under(5, bound - 1) - sum_under(15, bound - 1)
}

#[cfg(test)]
fn sum_under(k: u64, n: u64) -> u64 {
    let p: u64 = n / k;
    k * p * (p + 1) / 2
}
