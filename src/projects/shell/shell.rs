#![allow(dead_code)]

use std::{
    env,
    io::{stdin, stdout, Write},
    process::Command,
};

pub fn main_shell() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("❌ ERROR while reading input"); // blocks until the user pressed 'Enter' key

        let command = user_input.trim().split_whitespace().next().unwrap();
        let arguments = user_input.trim().split_whitespace();

        let my_os = env::consts::OS;

        // Windows does special things when it comes to shell
        // we need to go to cmd first before run the command shell
        if my_os == "windows" {
            let mut child_process = Command::new("cmd")
                .arg("/C")
                .arg(command)
                .args(arguments)
                .spawn()
                .expect("❌ FAILED to run the command");

            child_process.wait().expect("command wasn't running...");
        } else {
            let mut child_process = Command::new(command)
                .arg(command)
                .args(arguments)
                .spawn()
                .expect("❌ FAILED to run the command");

            child_process.wait().expect("command wasn't running...");
        }
    }
}
