// mod stack_heap;
// mod vars;
// mod ownership;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
mod error_handling;
mod unit_test;
extern crate rust_lib_learn;

fn main() {
    // println!("Hello, world!");
    // vars::run();
    // vars::sub_a::func_a();
    // vars::sub_b::func_b();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    error_handling::run();
    let rnd_num = rust_lib_learn::print_random_number();
    println!("Random number from rust_lib_learn: {}", rnd_num);
}
