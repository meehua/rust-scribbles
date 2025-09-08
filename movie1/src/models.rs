use std::{default, fmt::Display, string};

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Movie {
    pub disc: usize,
    pub year: String,
    pub title: String,
    pub remark: Option<String>,
}

// impl Movie {   // 因为在结构体加了Default宏，自动实现new方法和Default trait，所以这俩段都用不上了
//     pub fn new() -> Self {
//         Movie {
//             disc: 0,
//             year: String::new(),
//             title: String::new(),
//             remark: None,
//         }
//     }
// }
// impl Default for Movie {  // 实现Default trait
//     fn default() -> Self {
//         Self::new()
//     }
// }
