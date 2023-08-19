use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal,
    cursor::{self, MoveTo},
    execute,
};
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let _terminal = terminal::enable_raw_mode().unwrap();

    let mut input = Vec::new();
    print!("$ ");
    io::stdout().flush().unwrap();

    loop {
        match event::read().unwrap() {
            crossterm::event::Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Enter => {
                    let reversed_input: String = input.iter().rev().collect::<String>().split_whitespace().rev().collect::<Vec<_>>().join(" ");
                    
                    let output = Command::new("zsh")
                        .arg("-c")
                        .arg(&reversed_input)
                        .output()
                        .unwrap();

                    print!("\n");
                    io::stdout().write_all(&output.stdout).unwrap();
                    io::stderr().write_all(&output.stderr).unwrap();

                    print!("\n$ ");
                    io::stdout().flush().unwrap();
                    input.clear();
                },
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                    }
                    redraw_prompt(&input);
                },
                KeyCode::Char(c) => {
                    input.push(c);
                    redraw_prompt(&input);
                },
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    return;
                },
                _ => {}
            },
            _ => {}
        }
    }
}

fn redraw_prompt(input: &Vec<char>) {
    let display_str: String = input.iter().rev().collect();
    execute!(io::stdout(), MoveTo(2, cursor::position().unwrap().1)).unwrap();
    print!("{}{}", " ".repeat(input.len()), "\r$ ");
    io::stdout().write(display_str.as_bytes()).unwrap();
    io::stdout().flush().unwrap();
}
