use crate::{
    models::{Movie, Role},
    services::{self, get_logged_in_role, get_users, login_success, logout},
};
use std::error::Error;

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

pub fn handle_add(
    disc: usize,
    year: &str,
    title: &str,
    remark: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(Role::Admin) => {
            let mut movies = services::read_from_json()?;
            let new_movie = Movie {
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_movie);
            services::write_to_json(&movies)?;
            println!("Movie added.");
        }

        _ => {
            println!("You need to log in as Admin to add a movie")
        }
    }
    Ok(())
}
