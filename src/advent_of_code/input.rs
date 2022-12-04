pub fn count<T: Fn(&str) -> bool>(input: &str, count_fn: T) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        if count_fn(line) {
            count += 1;
        }
    }

    count
}

pub fn sum<T: Fn(&str) -> i32>(input: &str, sum_fn: T) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += sum_fn(line);
    }

    sum
}