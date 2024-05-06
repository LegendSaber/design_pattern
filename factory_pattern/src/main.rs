mod factory;
mod abstract_factory;

use factory::test as factory_test;
use abstract_factory::test as af_test;

fn main() {
    println!("factory test:");
    factory_test();
    println!("abstract factory test:");
    af_test();
}
