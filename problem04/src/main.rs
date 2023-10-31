fn main() {
    let mut total = 0;

    for val_a in 1000..10000 {
        for val_b in 1000..10000 {
            let current = multiply(val_a, val_b);
            if check(current) {
                if total < current {
                    total = current;
                }
            }
        }
    }

    println!("{} is your palindrome.", total);
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn check(total: i32) -> bool {
    let str = total.to_string();
    if str.len() % 2 == 0 {
        let mut start = 0;
        let middle = str.len() / 2;
        let mut end = str.len() - 1;
        for _n in 0..middle {
            if str.chars().nth(start) != str.chars().nth(end) {
                return false;
            } else {
                start = start + 1;
                end = end - 1;
            }
        }
        return true;
    }
    return false;
}
