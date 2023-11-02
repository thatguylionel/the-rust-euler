#[allow(dead_code)]
pub fn execute() {
    let position = 10001;
    let mut current = 2;
    let mut numbers: Vec<i64> = Vec::new();
    numbers.push(current);
    current += 1;
    loop {
        let mut is_prime: bool = true;

        for number in numbers.iter() {
            if number != &current && current % number == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            numbers.push(current);
        }
        if numbers.len() == position {
            break;
        }
        current += 1;
    }

    println!(
        "The number in position {} is {}",
        numbers.len(),
        numbers[numbers.len() - 1]
    );
}
