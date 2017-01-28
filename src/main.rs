mod eulers;
use eulers::two;

fn main () {
    let result = two::do_ex();
    println! ("Latest Euler is {0}", result.to_string());
}
