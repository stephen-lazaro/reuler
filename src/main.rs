#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]
#[macro_use] extern crate itertools;
mod eulers;

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
    println! ("Ninth Euler is {:?}", eulers::nine::do_ex())
}
