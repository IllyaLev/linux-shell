#[allow(unused_imports)]
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, ExecutableCommand, terminal::{Clear, ClearType, SetTitle}, cursor::MoveTo};
use std::{env, io};
#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::process::exit;
//use std::process::Command;
use hostname::get;
use chrono::Local;
use figlet_rs::FIGfont;
use std::fs;
use std::path::Path;

//* const ORANGE: Color = Color::Rgb { r: (((0xDC602E >> 16) & 0xFF) as u8), g: (((0xDC602E >> 8) & 0xFF) as u8), b: (((0xDC602E & 0xFF) as u8)) }; 
//* template of const color

#[allow(dead_code)]
const LIGHT_YELLOW: Color =  Color::Rgb { r: (((0xF7E733 >> 16) & 0xFF) as u8), g: (((0xF7E733 >> 8) & 0xFF) as u8), b: (((0xF7E733 & 0xFF) as u8)) };
const LIGHT_GREEN: Color = Color::Rgb { r: (((0x14CC60 >> 16) & 0xFF) as u8), g: (((0x14CC60 >> 8) & 0xFF) as u8), b: (((0x14CC60 & 0xFF) as u8)) };
#[allow(dead_code)]
const ORANGE: Color = Color::Rgb { r: (((0xDC602E >> 16) & 0xFF) as u8), g: (((0xDC602E >> 8) & 0xFF) as u8), b: (((0xDC602E & 0xFF) as u8)) };
const BLUE: Color = Color::Rgb { r: (((0x26408B >> 16) & 0xFF) as u8), g: (((0x26408B >> 8) & 0xFF) as u8), b: (((0x26408B & 0xFF) as u8)) };

#[allow(unused_variables)]
fn main() {

    // TODO: create ls function like dir in windows

    let title = "Linux Shell";

    stdout().execute(SetTitle(title)).unwrap();

    if let Err(e) = env::set_current_dir("C:\\"){
        eprintln!("Failed to set default directory: {}", e);
        exit(1);
    }

    let username = get_username();
    let hostname = get_hostname();

    let mut stdout = stdout();

    let mut commands_history: Vec<String> = Vec::new();
    let mut commands_history_time: Vec<String> = Vec::new();

    let font = FIGfont::standard().unwrap();

    let figure = font.convert("Linux Shell");

    print_highlighted(figure.unwrap().to_string(), Color::Green);

    loop{
        let user_and_host = format!("{}@{}\n", username, hostname);
        print_highlighted(user_and_host, Color::DarkGreen);

        stdout.execute(Print("$ ")).unwrap();

        let mut input = String::new();

        
        stdin().read_line(&mut input).expect("Failed to read line");

        let args: Vec<&str> = input.trim().split(' ').collect();

        if args[0] == ""{
            stdout.execute(Print("Please enter command. Don't just press enter!\n")).unwrap();
            continue;
        }

        let date_time_now = Local::now();
        let formatted_date_time_now = date_time_now.format("Date: |%Y|%m|%d|  Time: |%H|%M|%S|\n").to_string();

        //stdout.execute(Print(&formatted_date_time_now)).unwrap();

        commands_history.push(input.to_string());
        commands_history_time.push(formatted_date_time_now);

        if args[0] == "exit"{
            exit(0);
        }
        else if args[0] == "hmt"{
            get_commands_history(args, &commands_history, &commands_history_time);
        }
        else if args[0] == "about"{
            about();
        }
        else if args[0] == "echo"{
            let input = input.trim();
            let output = &input[5..];
            stdout.execute(Print(output))
                .unwrap()
                .execute(Print("\n"))
                .unwrap();
        }
        else if args[0] == "clear"{
            stdout.execute(Clear(ClearType::All))
                .unwrap()
                .execute(MoveTo(0,0))
                .unwrap()
                .execute(Clear(ClearType::Purge))
                .unwrap();
        }
        else if args[0] == "help"{
            help();
        }
        else if args[0] == "ls"{
            let path = if args.len() == 1 || args[1].is_empty() { "." } else { args[1] };
            let _ = ls(path);
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
            let text = format!("{}\n", commands_history_time[idx]);
            stdout.execute(Print(format!("-> {}   ", commands_history[idx]))).unwrap();
            print_highlighted(text, LIGHT_YELLOW);
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

fn about(){
    let text = "
    Welcome! This is linux shell, project that implement linux commands and some unique commands for Windows\n
    This shell is created by only using Rust\n
    Project is developed by Illia Levadskyi, who is studying in high school right now.
\n";
    print_highlighted(text.to_string(), LIGHT_GREEN);
}

fn help(){
    let text = "
    Here is list of available commands:                                            \n
        1 echo - outputs text in shell                                             \n
        2 exit - exits shell                                                       \n
        3 about - gives information about project                                  \n
        4 hmt - outputs history of commands of current shell.                      \n
            Have 2 flags: -al and -alt                                             \n
                -al - outputs only command                                         \n
                -alt - outputs command + date and time of it's execution           \n
        5 ls - outputs what contain given directory                                \n
            if you write just ls outputs what contain current directory            \n
            if you write ls + directory that will output contain of given directory\n\n";
    print_highlighted(text.to_string(), ORANGE);
}

fn ls(path: &str) -> io::Result<()>{
    let dir_path = Path::new(path);

    if !dir_path.is_dir() {
        eprintln!("The specified path is not a directory.");
        return Ok(());
    }
    
    println!("Listing contents of directory: {}", dir_path.display());

    let mut idx: u8 = 0;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if file_type.is_dir() {
            print_highlighted(format!("{}/   ", name), BLUE);
        }
        else if file_type.is_file(){
            print!("{}   ", name);
        }
        else{
            print_highlighted(format!("{}   ", name), ORANGE);
        }
        idx += 1;
        if idx == 3{
            println!();
            idx = 0;
        }
    }
    if idx != 0 {
        println!();
    }

    Ok(())
}