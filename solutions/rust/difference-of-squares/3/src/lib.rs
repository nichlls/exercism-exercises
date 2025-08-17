pub fn square_of_sum(n: u32) -> u32 {
    // gauss addition
    let sum = (n * (n + 1)) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // formula
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);

    square_of_sum - sum_of_squares
}
