use std::error::Error;

use crate::services::get_users;

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("User {} logged in", username);
    if let Some(user) = get_users()
        .iter()
        .find(|u| u.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter the password:");
        match rpassword::read_password() {
            Ok(password) => {
                println!("password: {password}");
                if user.password == password {
                    println!("Login successful!");
                } else {
                    println!("Incorrect password.");
                }
            }
            Err(_) => {
                println!("Failed to read password.");
            }
        }
    } else {
        println!("User not found.");
    }
    Ok(())
}
