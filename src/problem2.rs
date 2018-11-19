fn sum_of_even_fibonnaci() -> i64 {
    let mut sum: i64 = 0;
    let (mut curr, mut next): (i64, i64) = (0, 1);

    loop {
        let fibonnaci_num:i64 = curr + next;
        if fibonnaci_num > 4000000 {
            break;
        }

        if fibonnaci_num % 2 == 0 {
            sum += fibonnaci_num;
        }

        curr = next;
        next = fibonnaci_num;

    }

    sum
}

#[test]
fn test_sum_of_even_fibonnaci() {
    let sum: i64 = sum_of_even_fibonnaci();
    assert_eq!(sum, 4613732);
}