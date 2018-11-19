fn get_sum_of_multiples_of_3_and_5(number: i32) -> i32 {
    let mut sum: i32 = 0;

    for n in 1..number {
        if n % 3 == 0 {
            sum += n;
            continue;
        }

        if n % 5 == 0 {
            sum += n;
            continue;
        }
    }

    sum
}

#[test]
fn test_get_sum_of_multiples_of_3_and_5() {
    let sum = get_sum_of_multiples_of_3_and_5(1000);
    assert_eq!(sum, 233168);
}