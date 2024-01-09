use std::io::{self, stdin, stdout, Read, Write};
use rand::Rng;
use std::fs::OpenOptions;

fn main() {
    println!("Select whether you want to create or view passwords");
    println!("1: create new password, 2 = view password");
    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("failed to read");

    let ans: u32 = ans.trim().parse().expect("please choose an option");

    match ans {
        1 => pw().expect("Error generating password"),
        2 => viewer(),
        _ => println!("Invalid option"),
    }
    pause();
}

fn pw() -> io::Result<()> {
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

fn viewer() {
    let data = std::fs::read_to_string("./password.txt").expect("Error reading password file");
    println!("{}", data);
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
