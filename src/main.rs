#![allow(unused)]
use std::fs;
use std::io::{self, stdin, Read, Write};
use std::process::Command;

fn default(file_path: &str, dir: &str) {
    let mut response = Command::new("ffmpeg")
        .args(["-i", &file_path.trim(), &format!("{}/frame%04d.png", dir)])
        .spawn()
        .expect("Failed to spawn ffmpeg");

    response.wait().expect("Failed to wait for ffmpeg");
}

fn main() {
    print!("Enter Video Path:");
    let mut vid_path = String::new();
    io::stdout().flush().unwrap();
    stdin().read_line(&mut vid_path).unwrap();

    print!("Enter Directory Name to Store ( Creates a Directory ) :");
    io::stdout().flush().unwrap();
    let mut dir_name = String::new();
    stdin().read_line(&mut dir_name).unwrap();

    let dir_name = dir_name.trim();
    match fs::create_dir(dir_name) {
        Ok(_) => println!("Directory '{}' created successfully.", dir_name),
        Err(err) => println!("Error creating directory: {}", err),
    }
    default(&vid_path, dir_name);
}
