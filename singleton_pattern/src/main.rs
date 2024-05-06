mod single_object;
mod singleton;

use single_object::test as so_test;
use singleton::test as s_test;


fn main() {
    so_test();
    s_test();
}
