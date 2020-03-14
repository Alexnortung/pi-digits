use rug::float::Round;
use rug::ops::DivAssignRound;
use rug::Float;


fn leibniz(precision: u32) -> Float {
    let _zero = Float::with_val(precision, 0);
    let mut pi = Float::with_val(precision, 0);
    let mut i = 0;

    loop {
        let mut next = Float::with_val(precision, 4);
        let next_div = Float::with_val(precision, i * 2 + 1);
        let _next_dir = next.div_assign_round(next_div, Round::Zero);
        if i % 175000 == 0 {
            let exp = next.get_exp().unwrap();
            if exp.abs() > precision as i32 {
                break;
            }
            
            //println!("{}, {}", pi, next);
            println!("pi: {}, iteration: {}", pi, i);
        }

        if i % 2 == 0 {
            pi += next;
        } else {
            pi -= next;
        }
        i += 1;
    }
    pi
}

fn main() {
    let precision = 800;
    let _one = Float::with_val(precision, 1);
    let pi = leibniz(precision);

    let pi_string = pi.to_string_radix(10, None);
    let mut pi_string = pi.to_string_radix(10, Some(pi_string.len() - 3));
    pi_string.pop();
    println!("{}", pi_string);
}
