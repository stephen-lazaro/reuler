mod eulers;
use eulers::three;

fn main () {
    let result = three::do_ex();
    println! ("Latest Euler is {0}", result.to_string());
}
