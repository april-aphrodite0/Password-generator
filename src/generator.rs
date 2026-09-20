use std::io;
use std::io::Write;
use rand::rng;
use rand::RngExt;
use rand::seq::SliceRandom;

pub fn createpassword() {
    // ASK FOR LENGTH
    print!("how long? ");
    io::stdout().flush().unwrap();
    let mut length_input = String::new();
    io::stdin().read_line(&mut length_input).expect("failed to read line");
    let length: usize = match length_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid length");
            return;
        }
    };

    if length < 8 {
        println!("length must be at least 8");
        return;
    }

    // ASK FOR EACH CATEGORY
    let lowercase = ask_category("lowercase letters");
    let uppercase = ask_category("uppercase letters");
    let numbers   = ask_category("numbers");
    let symbols   = ask_category("symbols");

    // COUNT REQUIRED CHARACTERS
    let mut required: usize = 0;
    if let Some(n) = lowercase { required += n; }
    if let Some(n) = uppercase { required += n; }
    if let Some(n) = numbers   { required += n; }
    if let Some(n) = symbols   { required += n; }

    if required > length {
        println!(
            "you asked for {} required characters but the length is only {}",
            required, length
        );
        return;
    }

    // BUILD THE POOL FROM EVERY ALLOWED CATEGORY
    let mut pool = String::new();
    if lowercase.is_some() { pool.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if uppercase.is_some() { pool.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if numbers.is_some()   { pool.push_str("0123456789"); }
    if symbols.is_some()   { pool.push_str("!@#$%^&*()"); }

    if pool.is_empty() {
        println!("you need at least one character type");
        return;
    }

    // GENERATE
    let pool_chars: Vec<char> = pool.chars().collect();
    let mut rng = rng();
    let mut password_chars: Vec<char> = Vec::new();

    // ADD EXACT COUNTS
    add_exact(&mut password_chars, lowercase, "abcdefghijklmnopqrstuvwxyz", &mut rng);
    add_exact(&mut password_chars, uppercase, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", &mut rng);
    add_exact(&mut password_chars, numbers,   "0123456789",               &mut rng);
    add_exact(&mut password_chars, symbols,   "!@#$%^&*()",               &mut rng);

    // FILL REMAINING SLOTS RANDOMLY FROM THE POOL
    while password_chars.len() < length {
        let idx = rng.random_range(0..pool_chars.len());
        password_chars.push(pool_chars[idx]);
    }

    password_chars.shuffle(&mut rng);

    let password: String = password_chars.into_iter().collect();
    println!("{}", password);
}

// Returns None if not allowed, Some(0) for random, Some(n) for exactly n
fn ask_category(name: &str) -> Option<usize> {
    if !ask_yes_no(&format!("allow {}?", name)) {
        return None;
    }

    loop {
        print!("how many {}? (0 = random): ", name);
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("failed to read line");
        match answer.trim().parse::<usize>() {
            Ok(n) => return Some(n),
            Err(_) => println!("please enter a number"),
        }
    }
}

fn add_exact(
    password: &mut Vec<char>,
    count: Option<usize>,
    chars: &str,
    rng: &mut impl RngExt,
) {
    if let Some(n) = count {
        if n > 0 {
            let pool: Vec<char> = chars.chars().collect();
            for _ in 0..n {
                password.push(pool[rng.random_range(0..pool.len())]);
            }
        }
    }
}

fn ask_yes_no(question: &str) -> bool {
    loop {
        print!("{} (y/n): ", question);
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("failed to read line");
        match answer.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("please answer y or n"),
        }
    }
}