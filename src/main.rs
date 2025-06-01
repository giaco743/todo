use std::path::PathBuf;

use clap::Parser;
use todo::{todos, Todo, TodoInCode};
use walkdir::WalkDir;

#[derive(Parser)]
struct CliArgs {
    path: PathBuf,
    #[arg(short, long)]
    extensions: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let walker = WalkDir::new(args.path).into_iter();
    let files = walker
        .filter_map(|e| e.map(|e| e.path().to_path_buf()).ok())
        .filter(|path| {
            if args.extensions.is_empty() {
                path.is_file()
            } else if let Some(ext) = path.extension() {
                args.extensions
                    .contains(&ext.to_str().expect("Extension is Unicode").to_string())
            } else {
                false
            }
        })
        .collect();

    for TodoInCode { file, line, todo } in todos(files)? {
        let Todo { ticket_id, message } = todo;
        println!("{file:?}: {line} -> {ticket_id}: {message:?}");
    }
    Ok(())
}
