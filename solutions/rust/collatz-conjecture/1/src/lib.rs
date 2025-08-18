pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut num = n;
    let mut count = 0;

    while num != 1 {
        count += 1;
        if num % 2 == 0 {
            num /= 2;
        } else {
            num *= 3;
            num += 1;
        }
    }

    Some(count)
}
