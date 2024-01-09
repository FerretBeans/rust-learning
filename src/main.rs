use std::io::{self, stdin, stdout, Read, Write};
use rand::Rng;
use std::fs::OpenOptions;

fn main() {
    println!("Select whether you want to create or view passwords");
    println!("1: create new password, 2 = view password, 3 = exit the program");

    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("failed to read");

    let ans: u32 = ans.trim().parse().expect("please choose an option");

    match ans {
        1 => {
            pw().expect("Error generating password");
            main();
        },
        2 => {
            viewer();
            main();
        },
        3 => close(),
        _ => {
            println!("Invalid option");
            main();
        },
    }
}

fn pw() -> io::Result<()> {
    let char_arr = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$*!@#";

    let mut email = String::new();
    println!("Enter Email address");
    io::stdout().flush()?;
    io::stdin().read_line(&mut email)?;

    let mut num = String::new();
    println!("Enter the amount: ");
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

    let emailtxt = email.clone();
    let passwordtxt = password.clone();
    let mut writer = OpenOptions::new()
        .create(true)
        .append(true)
        .open("password.txt")?;
    writeln!(writer, "{}{}{}", emailtxt, passwordtxt, "\n")?;

    Ok(())
}

fn viewer() {
    let data = std::fs::read_to_string("./password.txt").expect("Error reading password file");
    println!("{}", data);
}

fn close() {
    std::process::exit(0);
}