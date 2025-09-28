use colored::*;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> std::io::Result<()> {
    // TODO: Create a loop and break this program into a few different files
    //

    // Enter alternate screen and clear it
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    let title = String::from("\n  Welcome to PERRITABONITA!  ");
    let subtitle = String::from("\n  A Rust program made with love ❤️  ");
    let dedication = String::from("\n  Inspired by our beautiful, crazy dog :)  ");
    let line = "\n-------------------------------------------------".bright_yellow();

    // Using a simple color output crate here  below

    println!("{}", title.red().on_bright_yellow());
    println!("{}", subtitle.red().on_bright_white());
    println!("{}", dedication.blue().on_bright_white());
    println!("{}", line);
    println!("\nType something for Perritabonita to say:\n");

    let picture = String::from(
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⠤⠤⠴⠒⠒⠒⠶⠶⠶⠤⠤⠴⠶⠶⠤⠤⠤⠤⠤⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠟⠉⢀⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣈⠱⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⠟⢁⣠⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣷⡈⠛⢦⣄⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢀⡴⠻⠋⣺⣽⡿⠀⠀⠀⠀⣀⡀⠀⠀⠆⠀⠀⠀⠀⠀⠀⢠⣴⣦⣀⠀⠀⠀⠘⣿⣦⡀⠙⢷⣄⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣀⣴⠟⠁⡠⣴⡇⡸⠁⠀⢠⣾⣿⣿⣿⣷⣠⠀⠀⠀⠀⠀⠀⠀⢼⣿⣿⣿⣿⣦⠀⠀⢸⠹⣷⣄⠀⠙⣿⡀⠀⠀⠀⠀
⠀⠀⢰⡞⠉⠁⠀⠤⠊⢠⡿⢱⠇⠀⠐⢛⡿⠿⠿⢿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠈⢿⠛⣿⣿⠛⠁⠀⠹⡷⣿⡇⠳⢄⡈⠳⢶⣦⡀⠀
⠀⠠⣾⠃⠀⡀⠀⠀⠀⢸⡇⣿⠀⠀⠀⠀⠀⠀⣠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢢⠀⠀⠀⠀⠀⠀⠹⣿⡇⠀⢄⢈⠢⣀⣿⡷⠦
⠀⣼⡷⠀⢀⠔⠂⡰⠁⢈⡇⡇⠀⠀⠀⠀⢀⡴⠁⠀⠀⣠⣶⣶⣿⣿⣿⣶⣦⡀⠀⠀⠳⡀⠀⠀⠀⠀⠀⣿⣧⡀⠀⡙⠱⣌⢻⣿⡂
⠰⢿⣧⠞⢁⡤⠊⠀⠀⣼⣇⡇⠀⡠⠔⠊⠁⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠈⠑⠢⢄⠀⢠⣿⡟⡇⠠⡘⢦⠈⢺⣯⠁
⠀⠸⣿⠶⠋⣠⠎⢀⡞⠙⣿⡇⠀⡇⣸⡀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⣀⠈⡆⣸⣿⡻⠀⠀⠻⣄⡳⣼⣿⡀
⠀⠀⢿⣶⠏⢁⣶⠟⠀⠀⣿⣧⠀⠀⣿⣿⡄⠀⠀⠀⠀⠀⠈⠉⢻⣿⣛⠉⠀⠀⠀⠀⠀⢠⣾⣿⡆⠁⣿⡟⡇⠀⣦⡑⠂⣱⣿⣿⠇
⠀⠀⠈⢻⣷⡊⠀⢀⣀⢤⢿⡟⢴⠄⠘⣻⣿⣄⡀⠀⠀⠀⣀⣠⢿⡿⣄⣀⡀⠀⠀⠠⣴⢿⣿⡿⠁⢰⣿⡇⢣⠀⢌⣣⣷⠟⠁⠀⠀
⠀⠀⠀⠀⠙⣿⣿⣋⡿⢫⣿⣧⣰⠀⠀⢇⢻⣏⠉⡛⡟⠋⠉⠀⢸⡇⠀⠉⠉⣻⣿⠟⠁⢰⡟⠀⠀⣼⣿⣇⢠⡇⣸⠟⠏⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠘⠿⠿⠶⣿⢿⣿⡏⡄⠀⠈⣶⢿⡀⢸⣷⠀⠀⠀⠸⠀⠀⠀⢀⣿⡟⠀⢠⡿⠀⠀⣄⠹⠻⣿⣄⣹⠏⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣰⢸⡏⠀⢼⡟⢡⡀⠀⠸⣈⢷⡈⢻⡄⠀⠀⠀⠀⠀⠀⢸⡟⠁⢠⣿⠃⢸⣾⣿⠀⠀⢻⡙⠏⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢉⢸⠁⠀⠘⢇⣾⠁⠀⠀⠙⡆⠳⡌⢷⡀⠀⠀⠀⠀⣠⡟⣠⣶⡿⠃⠈⡈⠛⣇⠀⠀⢸⠧⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⣸⡆⠀⠀⢸⣡⣶⣇⠀⠀⠈⢦⡙⢦⡛⠶⠦⠤⠖⣫⣾⣿⡿⠀⣤⢠⣇⠀⠁⠀⢠⢿⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣏⢿⡄⠀⠘⢻⣿⠿⡇⠀⠀⠀⠉⠪⣿⣿⣶⣶⣾⣟⡩⠀⠀⠀⣿⣼⡟⠀⠀⠀⡞⣤⠇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠘⡌⣷⡀⠀⠈⢻⣴⣷⣶⠀⠀⠀⡀⠀⠉⠚⠓⠚⠁⠀⠀⠀⠐⣿⠿⠀⠀⠀⡸⢁⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣎⠃⠀⠀⠀⠋⢃⣸⡺⠂⠀⣧⡼⢷⡄⢠⣄⡄⢀⣼⡟⣰⠻⠀⠀⠀⡰⢡⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠣⡀⠀⠀⠀⠛⠙⠁⠀⠘⠛⠁⢻⣿⠋⣿⡧⠈⢈⡴⠁⠀⠀⠀⠀⢁⠜⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⠉⠀⠙⠁⠰⠋⠀⠀⠀⠀⠀⠴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠒⠲⠀⠀⡀⠰⠐⠁⠐⡀⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    );

    let mut quote = String::new();

    let ending = "!".to_string();

    io::stdin().read_line(&mut quote).expect("What????");

    let new_quote = format!("{}{}", quote.to_uppercase().trim(), ending.trim());

    println!("{}", line);

    let message = String::from("\n  Perritabonita says:\n\n");

    println!("  {}", message);
    println!("  \"{}\"", new_quote.trim());
    println!("\n\n{}\n\n", picture.yellow());

    // Wait for user to press any key before exiting
    println!("Press Enter to exit...");
    let mut exit_input = String::new();
    io::stdin()
        .read_line(&mut exit_input)
        .expect("Failed to read line");

    // Restore terminal state
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
