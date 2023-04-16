use std::env;
use std::path::Path;

use std::process::Command;


fn main() {
    let args: Vec<String> = env::args().collect();

    let args = if args.len() > 1 {
        &args[1..]
    }
    else {
        &args[1..]
    };
    let winitrc = &format!("{}/.winitrc",env::var("HOME").unwrap());
    let mut command = if Path::new(&winitrc).exists() {
        //println!("Running ~/.winitrc");
        Command::new("sh")
            .stdin(std::fs::File::open(&winitrc).unwrap())
            .args(&args[..])
            .spawn().expect("Failed to execute ~/.winitrc")
    }
    else {
        Command::new(&args[0])
            .args(&args[1..])
            .spawn().expect("Failed to execute command")
    };

    command.wait().expect("Failed to wait on child");

}

