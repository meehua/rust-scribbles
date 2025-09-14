use crate::{
    models::{Movie, Role},
    services::{self, get_logged_in_role, get_users, login_success, logout},
};
use std::{error::Error, io::{self, Write}};

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

pub fn handle_delete(disc: usize, index: usize) -> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()?
     {
        let movies= services::read_from_json()?;
        if let Some(movie)=movies.iter()
            .filter(|m| m.disc==disc).enumerate()
            .find(|(i,_)| *i==index)
            .map(|(_,m)| m.clone() )
            {
                let left_movies=movies
                    .into_iter()
                    .filter(|m| *m!=movie)
                    .collect::<Vec<Movie>>();
                services::write_to_json(&left_movies)?;
                println!("Movie deleted.");
            }
    }else {
        println!("You need to log in as Admin to delete a movie");
    }
    Ok(())
}

pub  fn handle_edit(disc: usize,index: usize)-> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()?
     {
        let mut movies= services::read_from_json()?;
        if let Some(movie)=movies
            .iter_mut()// 生成可变的迭代器
            .filter(|m| m.disc==disc) // 过滤出某碟片的所有电影
            .enumerate() // 为每个元素生成索引
            .find(|(i,_)| *i==index) // 寻找用户指定的索引数据
            .map(|(_,m)| m ) // 返回该索引对应的影片信息
            {
                print!("Enter the new disc no.:");
                io::stdout().flush()?; // 输出终端缓存区的数据（防止信息滞留，用户看不到）
                let mut disc=String::new();
                io::stdin().read_line(&mut disc)?;
                let disc=disc.trim();
                if let Ok(disc) = disc.parse(){
                    movie.disc=disc;
                }else{
                    println!("Invalid disc number.");
                    return Ok(());
                }
                print!("Enter the new year:");
                io::stdout().flush()?;
                let mut year=String::new();
                io::stdin().read_line(&mut year)?;
                let year=year.trim();
                if year.parse::<usize>().ok().is_some(){
                    movie.year=year.to_string();
                }else{
                    println!("Invalid year.");
                    return Ok(());
                }

                print!("Enter the new title:");
                io::stdout().flush()?;
                let mut title=String::new();
                io::stdin().read_line(&mut title)?;
                let title=title.trim();
                if !title.is_empty(){
                    movie.title=title.to_string();
                }else{
                    println!("Invalid title.");
                    return Ok(());
                }

                print!("Enter the new remark (optional):");
                io::stdout().flush()?;
                let mut remark=String::new();
                io::stdin().read_line(&mut remark)?;
                let remark=remark.trim();
                if !remark.is_empty(){
                    movie.remark=None;
                }else{
                    movie.remark=remark.to_string().into(); // into() 方法将数据自动转换为需要的类型
                }

                services::write_to_json(&movies)?;
                println!("Movie modified.");
            }
    }else {
        println!("You need to log in as Admin to edit a movie");
    }
    Ok(())
}