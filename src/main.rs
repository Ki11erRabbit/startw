use std::env;
use std::path::Path;

use std::process::Command;
use std::os::unix::process::CommandExt;


fn main() {
    let args: Vec<String> = env::args().collect();

    if Path::new("~/.winitrc").exists() {
        Command::new("/bin/sh")
            .args(&args[1..])
            .exec();
    }
    else {
        Command::new(&args[1])
            .args(&args[2..])
            .exec();
    }

}

