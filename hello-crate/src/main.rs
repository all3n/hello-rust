mod hello;
use hello::{hello_public};
// use crate::hello::hello2;
// use hello_crate::hello::hello2;
// use crate::hello::hello_public;

use crate::hello::hello2::test_hello2 as test_hello3;


fn main() {
    hello_public();
    hello::hello_public();
    test_hello3();
}
