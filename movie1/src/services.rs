use std::error::Error;
use std::fs;

use crate::models::{Role, User};

pub fn get_users() -> Vec<User> {
    vec![
        User {
            username: "Admin".to_string(),
            password: "admin".to_string(),
            role: Role::Admin,
        },
        User {
            username: "Miha".to_string(),
            password: "miha".to_string(),
            role: Role::User,
        },
    ]
}

pub fn login_success(role: &Role) -> Result<(), Box<dyn Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_role() -> Result<Option<Role>, Box<dyn Error>> {
    let role = fs::read_to_string(".session")?;
    match role.as_str() {
        "Administrator" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None),
    }
}
