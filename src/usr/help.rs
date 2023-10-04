use std::process::ExitCode;

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    if args.len() > 1 {
        Ok(())
    } else {
        help_summary()
    }
}

fn help_summary() -> Result<(), ExitCode> {
    let cmds = [
        "help - print this help message",
        "exit - exit the shell",
        "ping - pong",
    ];
    for cmd in cmds.iter() {
        println!("{}", cmd);
    }
    Ok(())
}
