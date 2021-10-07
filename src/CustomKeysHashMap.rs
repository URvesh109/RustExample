//Alternate Custom key

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username {}", username);
    println!("Password {}", password);
    println!("Attempting Login...");
    println!();
    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successfully logon!");
            println!("Name {}", account_info.name);
            println!("Email {}", account_info.email);
            println!();
        }
        _ => println!("Login Failed"),
    }
}

fn main() {
    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "Urvesh109",
        password: "password@123",
    };

    let account_info = AccountInfo {
        name: "Urvesh Ghoderao",
        email: "urveshi.t@gmail.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "Urvesh109", "password123");
    try_logon(&accounts, "Urvesh109", "password@123");
}
