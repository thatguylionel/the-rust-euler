fn get_sided(a: f64, b: f64) -> f64 {
    let result = (a * a) + (b * b);
    return result.sqrt();
}

fn get_sqrt(item: f64) -> f64 {
    return item.sqrt();
}

fn calc_all_eq_triplet(a: f64, b: f64, c: f64, d: f64) -> bool {
    let result = a + b + c;
    return result == d;
}

fn main() {
    /*
      get the first value of a and loop. Inside the a loop, loop b. For each b loop, calc c, get its sqrt, add the 3 together and check for 12. loop 100 times
    */

    let mut a = 0;
    let mut b = 0;
    let mut c = 0.0;
    let triplet_answer = 1000.0;
    let mut is_matched: bool = false;
    for loop_a in 1..500 {
        a = loop_a;
        for loop_b in 1..500 {
            b = loop_b;
            if b > a {
                c = get_sided(a as f64, b as f64);
                if calc_all_eq_triplet(a as f64, b as f64, c, triplet_answer) {
                    is_matched = true;
                    break;
                } else if c > triplet_answer {
                    break;
                }
            }
        }
        if is_matched {
            break;
        }
    }

    if is_matched {
        println!(
            "{}² + {}² = {}²",
            get_sqrt(a as f64),
            get_sqrt(b as f64),
            c as i32
        );
        println!("{} + {} + {} = {}", a, b, c as i32, triplet_answer);
        println!("The product is {}", a * b * (c as i32));
    } else {
        println!("No match");
    }
}
