use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::env::args;
use std::fs;
use std::path::{Path, PathBuf};

// task data type
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    description: String,
}

// creating a task
impl Task {
    fn new(description: String) -> Task {
        Task {
            id: rand::thread_rng().gen_range(1000..=9999),
            description,
        }
    }
}

fn main() {
    let exe_path = env::current_exe().expect("not get the current exe path");
    let exe_dir = exe_path.parent().unwrap();
    let path: PathBuf = [exe_dir, Path::new("../../list.txt")].iter().collect();
    let mut tasklist: Vec<Task> = Vec::new();
    let args: Vec<String> = args().collect();
    let contents = fs::read_to_string(path.clone()).expect("failed to open file for reading");

    //deserializing data
    if !contents.is_empty() {
        tasklist = serde_json::from_str(&contents).expect("Failed to deserialize task list");
    }

    //finally the things that will be shown
    if args[1] == "--help" || args[1] == "-h" {
        println!("Usage:");
        println!("\tconfig [options]");
        println!("Options:");
        println!("\t--help, -h\tShow this help message");
        println!("\t--version, -v\tShow the version");
        println!("\t--list, -l\tList all configs");
        println!("\t--add, -a\tAdd a config");
        println!("\t--remove, -r\tRemove a config");
    } else if args[1] == "--version" || args[1] == "-v" {
        println!("version: {}", "0.1.0");
    } else if args[1] == "--list" || args[1] == "-l" {
        if tasklist.is_empty() {
            println!("No tasks found");
        } else {
            //
            //  TODO: sort things by data or priority instead of simply printing them
            //  TODO: or link a task title to a readme
            //
            println!("===========================================================================================");
            for task in tasklist.iter() {
                println!("id: {}  desc: {}", task.id, task.description);
            }
            println!("===========================================================================================");
        }
    } else if args[1] == "--add" || args[1] == "-a" {
        let task = Task::new(args[2].to_string());
        tasklist.push(task);
    } else if args[1] == "--remove" || args[1] == "-r" {
        let mut temp_tasklist: Vec<Task> = Vec::new();
        for task in tasklist.iter() {
            if task.id != args[2].parse::<usize>().unwrap() {
                temp_tasklist.push(task.clone());
            }
        }
        tasklist = temp_tasklist;
    } else {
        println!("usage: cargo run -- --add <description> or --remove <id> or --list");
    }

    //serializing data
    let serialized_data = serde_json::to_string(&tasklist).expect("Failed to serialize task list");
    fs::write(path, serialized_data).expect("Failed to write to file");
    println!("execution successful");
    println!(
        "Current UTC time: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S")
    );
}
