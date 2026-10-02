use std::env;
use std::fs;
use std::io::{self, BufRead, Read};
use std::path::Path;

enum Action {
    Steal,
    Restoration,
}

fn crawl(path: &str, files: &mut Vec<String>, ignore_list: &[String]) -> io::Result<()> {
    for item in fs::read_dir(path)? {
        let item = item?;
        let item_path = item.path();
        
        if let Some(name_os) = item_path.file_name() {
            let name = name_os.to_string_lossy();            
            if ignore_list.iter().any(|ignore| ignore == &name) {
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

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  ./main --steal [-i \"file1|folder1|...\"]");
        println!("  ./main --restore");
        return Ok(());
    }

    let mut ignore_list: Vec<String> = vec!["target".to_string(), ".git".to_string()];

    let mut current_action: Option<Action> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "steal" | "--steal" | "-steal" => {
                current_action = Some(Action::Steal);
            }
            "restore" | "--restore" | "-restore" => {
                current_action = Some(Action::Restoration);
            }
            "-i" | "--ignore" => {
                if i + 1 < args.len() {
                    let user_ignores = args[i + 1].split('|');
                    for item in user_ignores {
                        let trimmed = item.trim().to_string();
                        if !trimmed.is_empty() && !ignore_list.contains(&trimmed) {
                            ignore_list.push(trimmed);
                        }
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let action = match current_action {
        Some(act) => act,
        None => {
            println!("Invalid or missing action. Use '--steal' or '--restore'.");
            return Ok(());
        }
    };

    match action {
        Action::Steal => {
            let mut files = Vec::new();
            println!("Crawling target path with ignore list: {:?}", ignore_list);
            crawl(target_path, &mut files, &ignore_list)?;
            
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
