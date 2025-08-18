pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut sieve = vec![true; (n + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..=((n as f64).sqrt() as u64) {
        if sieve[i as usize] {
            for j in (i * i..=n).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }

    (0..=n).filter(|&i| sieve[i as usize]).collect()
}
