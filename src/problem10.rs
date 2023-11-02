pub fn execute() {
    let limit = 2000000;
    let mut primes: Vec<i64> = Vec::new();
    for number in 2..=limit {
        if is_prime(number, &primes) {
            primes.push(number);
        }
    }
    let total_sum: i64 = primes.iter().sum();
    println!(
        "The total sum of all primes below {} is {}",
        limit, total_sum
    );
}

fn is_prime(n: i64, primes: &[i64]) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for &prime in primes {
        if prime * prime > n {
            break;
        }
        if n % prime == 0 {
            return false;
        }
    }
    true
}
