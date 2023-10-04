mod usr;

fn main() {
    shell::init();
    usr::shell::spawn().expect("Something went wrong");
}
