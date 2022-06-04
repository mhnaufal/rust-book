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
mod projects;
mod r#struct;
mod variable;
mod vectors;

use challenges::armstrong;
use projects::shell::shell;

mod errors;
mod functional;
mod generic;
mod lifetime;
mod maps;
mod strings;
mod tests;
mod traits;

fn main() {
    /* CHAPTER 3 */
    variable::variable();
    data_type::data_type();
    function::main_function();
    control_flow::control_flow();

    /* CHAPTER 4 */
    ownership::ownership();
    borrowing::borrow();

    /* CHAPTER 5 & 6 */
    r#struct::r#struct();
    r#enum::r#enum();

    /* CHAPTER 8 */
    vectors::vector();
    strings::strings();
    maps::map();

    /* CHAPTER 9 & 10 */
    errors::errors();
    generic::generic();
    traits::traits();
    lifetime::lifetimes();

    /* CHAPTER 11 */
    // tests::tests();

    /* CHAPTER 13 */
    functional::functional();

    // PROJECTS
    shell::main_shell();

    /* CHALLENGES */
    challenges::hello_from_challenges(); // hello_from_challenge() placed inside the mod.rs file
    armstrong::is_armstrong_number(153); // is_armstrong_number() placed inside the armstrong.rs file
}
