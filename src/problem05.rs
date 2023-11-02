#[allow(dead_code)]
pub fn execute() {
    let mut current = 1i64;
    loop {
        let mut is_valid = true;
        for n in 1..21 {
            if current % n != 0 {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            println!("{} is the winner", current);
            break;
        }
        current = current + 1;
    }
}
