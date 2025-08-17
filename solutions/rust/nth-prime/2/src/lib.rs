pub fn nth(n: usize) -> u32 {
    if n == 0 {
        return 2;
    }

    // Set limit of primes
    let limit = if n < 6 {
        20
    } else {
        (n as f64 * (n as f64).ln() * 1.2) as usize
    };
    let mut primes = vec![true; limit];

    if primes.len() > 0 {
        primes[0] = false;
    }
    if primes.len() > 1 {
        primes[1] = false;
    }

    for i in 2..limit {
        if primes[i] {
            for j in (i * i..limit).step_by(i) {
                primes[j] = false;
            }
        }
    }

    // Find nth prime
    let mut count = 0;
    for i in 2..limit {
        if primes[i] {
            count += 1;
            if count == n + 1 {
                return i as u32;
            }
        }
    }

    panic!("Nth prime not found - {n}")
}

