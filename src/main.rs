#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]
#[macro_use] extern crate itertools;
#[allow(dead_code)]
#[allow(unused_import)]
mod eulers;

#[no_mangle]
pub fn one () -> i32 {
    eulers::one::do_ex()
}

#[no_mangle]
pub fn two () -> i32 {
    eulers::two::do_ex()
}

#[no_mangle]
pub fn three () -> i32 {
    eulers::three::do_ex() as i32
}

#[no_mangle]
pub fn five () -> i32 {
    eulers::five::do_ex () as i32
}

#[no_mangle] 
pub fn six () -> i32 {
    eulers::six::do_ex () as i32
}

#[no_mangle]
pub fn seven () -> i32 {
    eulers::seven::do_ex () as i32
}

#[no_mangle]
pub fn eight () -> i32 {
    eulers::eight::do_ex () as i32
}

#[no_mangle]
pub fn nine () -> [i32; 3] {
    let mut answer: [i32; 3] = [0, 0, 0];
    let nine = eulers::nine::do_ex();
    match nine {
        Some ((a, b, c)) => {
            answer[0] = a as i32;
            answer[1] = b as i32;
            answer[2] = c as i32;
            answer
        }
        None => answer
    }
}

#[no_mangle]
pub fn ten () -> i64 {
    eulers::ten::do_ex() as i64
}

fn main () {
    println! ("First Euler is {0}", eulers::one::do_ex().to_string());
    println! ("Second Euler is {0}", eulers::two::do_ex().to_string());
    println! ("Third Euler is {0}", eulers::three::do_ex().to_string());
    println! ("Fourth Euler is {:?}", eulers::four::do_ex());
    println! ("Examine gcd {}", eulers::five::gcd(27, 9));
    println! ("Fifth Euler is {}", eulers::five::do_ex());
    println! ("Sixth Euler is {}", eulers::six::do_ex());
    println! ("Seventh Euler is {}", eulers::seven::do_ex());
    println! ("Eigth Euler is {}", eulers::eight::do_ex());
    println! ("Ninth Euler is {:?}", eulers::nine::do_ex());
    println! ("Tenth Euler is {}", eulers::ten::do_ex());
    println! ("Eleventh Euler is {}", eulers::eleven::do_ex());
    println! ("Twelfth Euler is {}", eulers::twelve::do_ex());
}
