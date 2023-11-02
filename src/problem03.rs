#[allow(dead_code)]
pub fn execute() {
    /*
    <p>The prime factors of $13195$ are $5, 7, 13$ and $29$.</p>
    <p>What is the largest prime factor of the number $600851475143$?</p>
     */

    let mut total = 600851475143;
    let grand_total = total;
    let mut n = 1;
    //let pf = 13195;
    let mut numbers: Vec<i64> = Vec::new();
    let mut numbers_used: Vec<i64> = Vec::new();

    while n != 1000000 {
        n += 1;
        if n % 2 != 0 || n == 2 {
            numbers.push(n);
        }
    }

    let mut highest_number = 0;
    while total != 1 {
        //  Grab the first element of my prime factor list
        for number in numbers.iter() {
            if total % number == 0 {
                numbers_used.push(*number);
                if number > &highest_number {
                    highest_number = *number;
                }
                total = total / number;
                break;
            }
        }
    }

    println!("{} is the highest number", highest_number);

    for (i, prime) in numbers_used.iter().enumerate() {
        if i > 0 {
            print!(" * "); // Print a comma before all but the first number
        }
        print!("{}", prime);
    }
    print!(" = {}", grand_total);
}
