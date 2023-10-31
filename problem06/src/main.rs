fn main() {
    let mut numbers: Vec<i64> = Vec::new();
    let max = 100;
    for x in 1..max + 1 {
        numbers.push(x);
    }

    let total = sq_of_the_sum(numbers.clone()) - sum_of_the_sq(numbers.clone());
    println!("The difference between the sum of the squares of the first {} natural numbers and the square of the sum is {}",max, total);
}

fn sum_of_the_sq(numbers: Vec<i64>) -> i64 {
    let mut total = 0;
    for number in numbers.iter() {
        total = total + (number * number);
    }
    return total;
}

fn sq_of_the_sum(numbers: Vec<i64>) -> i64 {
    let mut total = 0;
    for number in numbers.iter() {
        total = total + number;
    }
    return total * total;
}
