pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 {
        return 0;
    }

    let mut count: usize = 0;
    
    let mut n = display_value;
    while n > 0 {
        // least significant bit
        count += (n & 1) as usize;
        // process next bit
        n >>= 1;
    }
    count
}
