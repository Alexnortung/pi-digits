use rug::float::Round;
use rug::ops::DivAssignRound;
use rug::Float;

use rug::Rational;
use rug::Integer;
use core::cmp::Ordering;

fn rational_to_float(ration: &Rational ,precision: u32) -> Float {
    let numer = ration.numer();
    let denom = ration.denom();
    let mut fl = Float::with_val(precision, numer);
    let _ = fl.div_assign_round(denom, Round::Zero);
    fl
}

fn _leibniz_rational(precision: u32) -> Float {
    let mut pi = Rational::from((0,1));
    let mut i = Integer::from(0);
    let four = Integer::from(4);
    let zero = Integer::from(0);
    
    loop {
        if i.mod_u(175000) == 0 {
            let pi_float = rational_to_float(&pi, precision);
            match pi_float.get_exp() {
                Some(num) if num > precision as i32 => break,
                _ => (),
            }
            println!("pi: {}, iteration: {}", pi_float, i);
        }
        let next_denom = Integer::from(&i * 2) + Integer::from(1);
        let next_rational = Rational::from((&four, &next_denom));
        if Integer::from(&i % 2).cmp(&zero) == Ordering::Equal {
            pi += next_rational;
        } else {
            pi -= next_rational;
        }

        i += 1;
    }

    rational_to_float(&pi, precision)
}

fn _leibniz(precision: u32) -> Float {
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

// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
//fn _chudnovski(precision: u32) -> Float {
//    let mut K = 6;
//    let mut X = 1;
//    let mut M = 1;
//    let mut L = 13591409;
//    let mut sum = 13591409;
//
//    loop {
//        
//        L += 545140134;
//        X *= -262537412640768000;
//        K += 12;
//        break;
//    }
//
//    Float::new(precision)
//}

fn main() {
    let precision = 20;
    let _one = Float::with_val(precision, 1);
    let pi = _leibniz_rational(precision);

    let pi_string = pi.to_string_radix(10, None);
    let mut pi_string = pi.to_string_radix(10, Some(pi_string.len() - 3));
    pi_string.pop();
    println!("{}", pi_string);
}
