#![allow(unused)]
use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;

fn default(file_path: &str, dir: &str) {
    let mut response = Command::new("ffmpeg")
        .args(["-i", &file_path.trim(), &format!("{}/frame%04d.png", dir)])
        .spawn()
        .expect("Failed to spawn ffmpeg");

    response.wait().expect("Failed to wait for ffmpeg");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <video_path> <directory_name>", args[0]);
        return;
    }

    let vid_path = &args[1];
    let dir_name = &args[2];
    let dir_path = Path::new(dir_name);

    if !dir_path.exists(){  // implemented the condition to check if the dir_path exists
    match fs::create_dir(dir_name) {
        Ok(_) => println!("Directory '{}' created successfully.", dir_name),
        Err(err) => {
            println!("Error creating directory: {}", err);
            return;
             }
        }
    }else{
        println!("Directory '{}' already exists. Using the existing directory.", dir_name);

    }

    default(vid_path, dir_name);
}
