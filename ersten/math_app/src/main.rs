use std::env;
use std::fs;
use std::io::{self, BufRead, Read};
use std::path::Path;

enum Action {
    Steal,
    Restoration,
}

fn crawl(path: &str, files: &mut Vec<String>, ignore_list: &[&str]) -> io::Result<()> {
    for item in fs::read_dir(path)? {
        let item = item?;
        let item_path = item.path();
        
        if let Some(name_os) = item_path.file_name() {
            let name = name_os.to_string_lossy();            
            if ignore_list.contains(&name.as_ref()) {
                continue;
            }
        }

        if item_path.is_dir() {
            let dir = item_path.to_string_lossy().into_owned();
            crawl(&dir, files, ignore_list)?;
        } else {
            let path_str = item_path.to_string_lossy().into_owned();
            files.push(path_str);
        }
    }
    Ok(())
}

fn stealer(list: &[String], folder: &str, replace: &str) -> io::Result<()> {
    fs::create_dir_all(folder)?;

    for file in list {
        if file.ends_with(".txt") {
            let existing = fs::read_to_string(file)?;

            let safe_backup_name = file.replace(['/', '\\'], "_");
            let destination = format!("{}/{}", folder, safe_backup_name);

            let backup_content = format!("{}\n{}", file, existing);
            fs::write(&destination, backup_content)?;

            fs::write(file, replace)?;
        }
    }
    Ok(())
}

fn restoration(backup_folder: &str) -> io::Result<()> {
    if !Path::new(backup_folder).exists() {
        return Ok(());
    }

    for item in fs::read_dir(backup_folder)? {
        let item = item?;
        let backup_file_path = item.path();

        if backup_file_path.is_file() {
            let file = fs::File::open(&backup_file_path)?;
            let mut reader = io::BufReader::new(file);
            
            let mut original_path_str = String::new();
            reader.read_line(&mut original_path_str)?;
            
            let original_path_str = original_path_str.trim_end_matches(['\r', '\n']);

            if !original_path_str.is_empty() {
                let mut remaining_bytes = Vec::new();
                reader.read_to_end(&mut remaining_bytes)?;

                if let Some(parent) = Path::new(original_path_str).parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(original_path_str, remaining_bytes)?;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let target_path = "/home/sakis/femboy_haven";
    let backup_folder = "/home/sakis/femboy_backup";
    let replace_text = "Taken 1-3\n";
    let ignore = vec!["target", ".git"];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  ./main --steal      (Crawls target, backs up, and replaces content)");
        println!("  ./main --restore    (Restores files from backup folder)");
        return Ok(());
    }

    let current_action = match args[1].as_str() {
        "steal" | "--steal" | "-steal" => Action::Steal,
        "restore" | "--restore" | "-restore" => Action::Restoration,
        _ => {
            println!("Invalid argument. Use '--steal' or '--restore'.");
            return Ok(());
        }
    };

    match current_action {
        Action::Steal => {
            let mut files = Vec::new();
            println!("Crawling target path...");
            crawl(target_path, &mut files, &ignore)?;
            
            println!("Backing up and replacing files...");
            stealer(&files, backup_folder, replace_text)?;
        }
        Action::Restoration => {
            println!("Restoring files from backup...");
            restoration(backup_folder)?;
        }
    }

    Ok(())
}
