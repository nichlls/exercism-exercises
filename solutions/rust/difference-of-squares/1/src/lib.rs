pub fn square_of_sum(n: u32) -> u32 {
    let mut total = 0;
    
    for i in 1..=n {
        total += i;
    }

    // square total
    total *= total;
    total
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut squares = vec![];

    for i in 1..=n {
        squares.push(i * i);
    }

    // sum of the squares
    let mut sum = 0;
    for i in squares {
        sum += i;
    }

    sum
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);

    square_of_sum - sum_of_squares
}
