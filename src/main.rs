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
use challenges::assembly;
mod strings;
mod maps;

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
    challenges::hello_from_challenges();    // hello_from_challenge() placed inside the mod.rs file
    armstrong::is_armstrong_number(153);  // is_armstrong_number() placed inside the armstrong.rs file
    assembly::production_rate_per_hour(12);
    strings::strings()
    maps::map();
}
