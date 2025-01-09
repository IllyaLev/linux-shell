#[allow(unused_imports)]
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::process::exit;
use hostname::get;
use chrono::Local;
use figlet_rs::FIGfont;


#[allow(unused_variables)]
fn main() {

    let username = get_username();
    let hostname = get_hostname();

    let mut stdout = stdout();

    let mut commands_history: Vec<String> = Vec::new();
    let mut commands_history_time: Vec<String> = Vec::new();

    let font = FIGfont::standard().unwrap();

    let figure = font.convert("Shell Linux");

    println!("{}", figure.unwrap());

    loop{
        let user_and_host = format!("{}@{}\n", username, hostname);
        print_highlighted(user_and_host, Color::DarkGreen);

        stdout.execute(Print("$ ")).unwrap();

        let mut input = String::new();

        
        stdin().read_line(&mut input).expect("Failed to read line");

        let args: Vec<&str> = input.trim().split(' ').collect();

        if input == ""{
            stdout.execute(Print("Please enter command. Don't just press enter!\n")).unwrap();
            continue;
        }

        let date_time_now = Local::now();
        let formatted_date_time_now = date_time_now.format("Date: |%Y|%m|%d|  Time: |%H|%M|%S|\n").to_string();

        stdout.execute(Print(&formatted_date_time_now)).unwrap();

        commands_history.push(input.to_string());
        commands_history_time.push(formatted_date_time_now);

        if args[0] == "exit"{
            exit(0);
        }
        else if args[0] == "hmt"{
            get_commands_history(args, &commands_history, &commands_history_time);
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
fn get_commands_history(args: Vec<&str>, commands_history: &[String], commands_history_time: &[String]){
    
    let mut stdout = stdout();
    let size = commands_history.len();
    if args[1] == "-al"{
        for idx in 0..(size - 1){
            stdout.execute(Print(format!("-> {}\n", commands_history[idx]))).unwrap();
        }
    }
    else if args[1] == "-alt"{
        for idx in 0..(size - 1){
            stdout.execute(Print(format!("-> {}   {}\n", commands_history[idx], commands_history_time[idx]))).unwrap();
        }
    }
}

fn print_highlighted(input: String, color: Color){
    let mut stdout = stdout();
    stdout.execute(SetForegroundColor(color))
        .unwrap()
        .execute(Print(input))
        .unwrap();
    stdout.execute(ResetColor).unwrap();
}