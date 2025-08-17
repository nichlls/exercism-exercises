pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("Square can not be zero")
    }
    
    let mut grain_count: u64 = 0;
    
    for i in 1..=s {
        grain_count *= 2;
        if grain_count == 0 {
            grain_count += 1;
        }
    }
    grain_count
}

pub fn total() -> u64 {
    let mut total = 0;
    
    for i in 1..=64 {
        total += square(i);
    }
    total
}
