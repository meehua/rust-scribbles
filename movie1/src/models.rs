use std::fmt::Display;

pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}

pub enum Role {
    Admin,
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Administrator"),
            Role::User => write!(f, "User"),
        }
    }
}
