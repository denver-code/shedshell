use crate::usr;
use shell::sys::execute::execute_command;
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
        // slit input into list
        let command: Vec<&str> = input.split(" ").collect();

        let res = match command[0] {
            "help" => usr::help::main(&[]),
            "exit" => usr::exit::main(),
            "ping" => usr::ping::main(),
            "" => Ok(()),
            _ => {
                // need to scan /bin folder for command and if exist - execute
                let command_path = std::path::Path::new("bin").join(format!("{}.shed", command[0]));
                if command_path.exists() {
                    match execute_command(command_path.to_str().unwrap(), &command[1..]) {
                        Ok(output) => println!("{}", output),
                        Err(e) => println!("Error executing command: {:?}", e),
                    }
                } else {
                    println!("Unknown command: {}", command[0]);
                }

                Ok(())
            }
        };

        if let Err(e) = res {
            println!("Error: {:?}", e);
        }
    }
}
