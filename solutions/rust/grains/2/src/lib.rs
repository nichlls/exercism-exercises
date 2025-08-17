pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("Square can not be zero")
    }

    1u64 << (s - 1)
}

pub fn total() -> u64 {
    let mut total = 0;
    
    for i in 1..=64 {
        total += square(i);
    }
    total
}
