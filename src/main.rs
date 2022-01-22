#![allow(dead_code)]

mod variable;
mod data_type;
mod function;
mod control_flow;
mod ownership;
mod borrowing;
mod r#struct;
mod r#enum;
mod vectors;
mod challenges;
use challenges::armstrong;

fn main() {
    variable::variable();
    data_type::data_type();
    function::main_function();
    control_flow::control_flow();
    ownership::ownership();
    borrowing::borrow();
    r#struct::r#struct();
    r#enum::r#enum();
    vectors::vector();
    challenges::hello_from_challenges();
    armstrong::is_armstrong_number(1);
}
