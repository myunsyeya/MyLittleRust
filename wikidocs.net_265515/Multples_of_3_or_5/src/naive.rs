#[test]
fn test() {
    let n: i32 = 1000;
    let mut answer: i32 = 0;

    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            answer += i;
        }
    }
    assert_eq!(233168, answer);
}
