use std::io::{self, Write};
use rand::Rng;
use std::fs::OpenOptions;

fn main() -> io::Result<()> {
    let char_arr = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$*!@#";
    let mut num = String::new();
    print!("Enter the amount: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut num)?;
    let num: usize = num.trim().parse().expect("Invalid input");

    let mut password = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        let pw = rng.gen_range(0..char_arr.len());
        password.push(char_arr.chars().nth(pw).unwrap());
    }
    println!("{}", password);

    let passwordtxt = password.clone();
    let mut writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open("password.txt")?;
    writeln!(writer, "{}", passwordtxt)?;

    Ok(())
}