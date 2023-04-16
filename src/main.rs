use std::env;
use std::path::Path;
use nix::unistd::execv;
use std::ffi::CString;

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

/*fn main() {
    let args: Vec<String> = env::args().collect();


    if Path::new("~/.winitrc").exists() {
        let mut args = args[1..].iter().map(|s| CString::new(s.as_str()).unwrap()).collect::<Vec<CString>>();
        args.insert(0,CString::new(env::var("HOME").unwrap() + "/.winitrc").unwrap());
        let path = CString::new("/bin/sh").unwrap();

        match execv(&path, &args) {
            Ok(_) => unreachable!(),
            Err(e) => println!("execv() returned Err: {}", e),
        }
    }
    else {
        // open first arg on path and make that the command to be executed
        let args = &args[1..];
        let command = lookup_command(&args[0]);

        let command = match command {
            Some(s) => s,
            None => {
                println!("Command not found");
                return;
            }
        };

        let args = args.iter().map(|s| CString::new(s.as_str()).unwrap()).collect::<Vec<CString>>();
        
        match execv(&CString::new(command.as_str()).unwrap(), &args) {
            Ok(_) => unreachable!(),
            Err(e) => println!("execv() returned Err: {}", e),
        }
    }
}

fn lookup_command(command: &str) -> Option<String> {
    let path = env::var("PATH").unwrap().split(":").map(|s| s.to_string() + "/" + &command).collect::<Vec<String>>();
    let commands = path.iter().map(|s| format!("{}/{}",s, command)).collect::<Vec<String>>();

    for command in commands {
        if Path::new(&command).exists() {
            return Some(command);
        }
    }
    None
}*/
