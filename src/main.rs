mod gui;

#[allow(unused_imports)]
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::process::exit;
use hostname::get;
use chrono::Local;


#[allow(unused_variables)]
fn main() {

    let username = get_username();
    let hostname = get_hostname();

    let mut stdout = stdout();

    let commands_history: Vec<String> = Vec::new();

    loop{
        let user_and_host = format!("{}@{}\n", username, hostname);
        stdout.execute(SetForegroundColor(Color::DarkGreen))
            .unwrap()
            .execute(Print(user_and_host))
            .unwrap();
        
        stdout.execute(ResetColor).unwrap();

        stdout.execute(Print("$ ")).unwrap();

        let mut input = String::new();

        
        stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == ""{
            stdout.execute(Print("Please enter command. Don't just press enter!\n")).unwrap();
            continue;
        }

        let date_time_now = Local::now();
        let formatted_date_time_now = date_time_now.format("Date: |%Y|%m|%d|  Time: |%H|%M|%S|\n").to_string();

        stdout.execute(Print(formatted_date_time_now)).unwrap();

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
#[allow(dead_code)]
fn get_commands_history(){

}