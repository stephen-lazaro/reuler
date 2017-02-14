mod eulers;

fn main () {
    println! ("First Euler is {0}", eulers::one::do_ex().to_string());
    println! ("Second Euler is {0}", eulers::two::do_ex().to_string());
    println! ("Third Euler is {0}", eulers::three::do_ex().to_string());
    prinln! ("Latest Euler is {0}", eulers::four::do_ex().to_string());
}
