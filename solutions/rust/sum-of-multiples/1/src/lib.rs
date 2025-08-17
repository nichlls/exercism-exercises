pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];

    for &factor in factors {
        let mut multiple = factor;
        while multiple < limit {
            multiples.push(multiple);
            multiple += factor;
        }
    }

    multiples.sort();
    multiples.dedup();
    multiples.into_iter().sum()
}
