#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod borrowing;
mod challenges;
mod control_flow;
mod data_type;
mod r#enum;
mod function;
mod ownership;
mod r#struct;
mod variable;
mod vectors;
use challenges::armstrong;
mod errors;
mod functional;
mod generic;
mod lifetime;
mod maps;
mod strings;
mod tests;
mod traits;

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
    challenges::hello_from_challenges(); // hello_from_challenge() placed inside the mod.rs file
    armstrong::is_armstrong_number(153); // is_armstrong_number() placed inside the armstrong.rs file
    strings::strings();
    maps::map();
    errors::errors();
    generic::generic();
    traits::traits();
    lifetime::lifetimes();
    // tests::tests();
    functional::functional();
}
