use crate::usr;
use std::{io::Write, process::ExitCode};

pub fn spawn() -> Result<(), ExitCode> {
    println!("Spawned shell");

    loop {
        print!("denver@shed:~$ ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        let input = input.trim();

        let res = match input {
            "help" => usr::help::main(&[]),
            "exit" => usr::exit::main(),
            "ping" => usr::ping::main(),
            "" => Ok(()),
            _ => {
                println!("Unknown command: {}", input);
                Ok(())
            }
        };

        if let Err(e) = res {
            println!("Error: {:?}", e);
        }
    }
}
