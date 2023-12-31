#[allow(dead_code)]
pub fn execute() {
    /*
    Each new term in the Fibonacci sequence is generated by adding the previous two terms.
    By starting with $1$ and $2$, the first $10$ terms will be:
    $$1, 2, 3, 5, 8, 13, 21, 34, 55, 89, \dots$$

    By considering the terms in the Fibonacci sequence whose values do not exceed four million,
    find the sum of the even-valued terms.
    */
    // Sequence cannot exceed 4,000,000
    let mut current_seq = 0;
    let mut previous_value = 0;
    let mut next_value = 1;
    let cap = 4000000;
    let mut grand_total = 0;

    //  Lets do the fibonacci
    while next_value < cap {
        // check if number is even
        if current_seq % 2 == 0 {
            grand_total = grand_total + current_seq;
        }
        current_seq = previous_value + next_value;
        previous_value = next_value;
        next_value = current_seq;
        if next_value >= cap {
            break;
        }
    }

    println!(
        "The total amount for all even numbers, with a cap of {}, is {}",
        cap, grand_total
    );
}
