// CHANGE HERE: Removed `File`
use std::fs::OpenOptions;
// CHANGE HERE: Removed `Seek` and `SeekFrom`
use std::io::{self, BufReader, BufWriter};
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Mark a task as complete
    Complete { id: u32 },
    /// Remove a task
    Remove { id: u32 },
}

const DB_FILE: &str = "tasks.json";

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    let mut tasks = load_tasks()?;

    match cli.command {
        Commands::Add { description } => {
            let new_id = tasks.last().map_or(1, |task| task.id + 1);
            let new_task = Task {
                id: new_id,
                description,
                completed: false,
            };
            tasks.push(new_task);
            println!("Added task with ID: {}", new_id);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                // CHANGE HERE: Iterate over a reference to `tasks`
                for task in &tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{} {}: {}", status, task.id, task.description);
                }
            }
        }
        Commands::Complete { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                println!("Task {} marked as complete.", id);
            } else {
                eprintln!("Error: Task with ID {} not found.", id);
            }
        }
        Commands::Remove { id } => {
            let initial_len = tasks.len();
            tasks.retain(|task| task.id != id);
            if tasks.len() < initial_len {
                println!("Task {} removed.", id);
            } else {
                eprintln!("Error: Task with ID {} not found.", id);
            }
        }
    }
    
    save_tasks(&tasks)?;

    Ok(())
}

fn load_tasks() -> io::Result<Vec<Task>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(DB_FILE)?;

    let reader = BufReader::new(file);
    
    match serde_json::from_reader(reader) {
        Ok(tasks) => Ok(tasks),
        Err(e) if e.is_eof() => Ok(Vec::new()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DB_FILE)?;

    let writer = BufWriter::new(&mut file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}