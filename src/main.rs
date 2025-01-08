#[allow(unused_imports)]
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
use std::process::exit;
use hostname::get;

fn main() {

    let username = get_username();
    let hostname = get_hostname();

    let mut stdout = stdout();
    
    loop{
        let user_and_host = format!("{}{}\n", username, hostname);
        stdout.execute(SetForegroundColor(Color::DarkGreen))
            .unwrap()
            .execute(Print(user_and_host))
            .unwrap();
        
        stdout.execute(ResetColor).unwrap();

        stdout.execute(Print("$ ")).unwrap();

        let mut input = String::new();
        
        stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit"{
            exit(0);
        }
    }
}

fn get_username() -> String{
    std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "unknown".to_string())
}

fn get_hostname() -> String{
    get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string())
}