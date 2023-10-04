use std::process::{Command, ExitCode};

pub fn execute_command(command_name: &str, args: &[&str]) -> Result<String, ExitCode> {
    let mut command = Command::new(command_name);
    command.current_dir(std::env::current_dir().unwrap());
    command.args(args);

    let output = command.output().unwrap();

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        println!("oops");
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}
