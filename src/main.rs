use std::path::{Path, PathBuf};

use clap::Parser;
use todo::{
    git,
    todos::{todos, TodoInCode},
};
use walkdir::WalkDir;

#[derive(Parser)]
struct CliArgs {
    path: PathBuf,
    #[arg(short, long)]
    extensions: Vec<String>,
    #[arg(short, long)]
    issue: Option<String>,
    #[arg(long)]
    git: Option<String>,
}

fn print_todos(todos: &Vec<TodoInCode>, issue: &Option<String>) {
    for TodoInCode {
        file,
        line,
        ticket_id,
        message,
    } in todos.into_iter().filter(|todo| {
        if let Some(issue) = issue {
            &todo.ticket_id == issue
        } else {
            true
        }
    }) {
        let file = file.to_str().expect("Filename is a valid unicode string");
        println!(
            "{file}:{line} -> {ticket_id}{}",
            message
                .as_ref()
                .map(|msg| format!(": {msg}"))
                .unwrap_or_default()
        );
    }
}

fn files<P: AsRef<Path>>(path: P, extensions: &Vec<String>) -> Vec<PathBuf> {
    let walker = WalkDir::new(path).into_iter();
    walker
        .filter_entry(|e| {
            if let Some(filename) = e.file_name().to_str() {
                !filename.starts_with(".") // skip .git
            } else {
                false
            }
        })
        .filter_map(|e| e.map(|e| e.path().to_path_buf()).ok())
        .filter(|path| {
            if extensions.is_empty() {
                path.is_file()
            } else if let Some(ext) = path.extension() {
                extensions.contains(&ext.to_str().expect("Extension is Unicode").to_string())
            } else {
                false
            }
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    println!(
        "Extract pending ToDos from codebase at {}",
        args.path.display()
    );
    let files = files(&args.path, &args.extensions);
    let todos = todos(files)?;

    if let Some(base_branch) = &args.git {
        println!(
            "Get all issue IDs which should be addressed by this branch compared to its base {base_branch}."
        );
        let resolving_issues = git::resolving_issues(&args.path, base_branch)?;
        let unresolved_todos: Vec<TodoInCode> = todos
            .into_iter()
            .filter(|todo| resolving_issues.contains(&todo.ticket_id))
            .collect();
        if !unresolved_todos.is_empty() {
            println!("Following todos associated with ticket IDs: {resolving_issues:?} are not yet resolved:");
            print_todos(&unresolved_todos, &None);
            return Err(anyhow::Error::msg("There are still unresolved ToDos!"));
        } else {
            println!("All todos associated with ticket IDs: {resolving_issues:?} are resolved.");
        }
    } else {
        println!(
            "Pending ToDos{} are:",
            args.issue
                .as_ref()
                .map(|issue| format!(" associated with issue {issue}"))
                .unwrap_or_default()
        );
        print_todos(&todos, &args.issue);
    }

    Ok(())
}
