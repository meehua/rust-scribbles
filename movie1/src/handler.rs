use std::error::Error;

use crate::services::{self, get_logged_in_role, get_users, login_success, logout};

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
                    login_success(&user.role)?;
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

pub fn handle_logout() {
    logout();
    println!("Logged out successfully.")
}

pub fn handle_list() -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(_) => {
            let movies = services::read_from_json()?;
            services::list_movies(&movies);
        }
        None => {
            println!("You need to log in to view the movie list.");
        }
    }
    Ok(())
}
