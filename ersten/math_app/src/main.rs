use std::env;
use night_crawler_lib::{crawl, get_home_dir, restoration, stealer};

enum Action {
    Steal,
    Restoration,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = get_home_dir();
    let target_path = home.join("femboy_haven");
    let backup_folder = home.join("femboy_backup");
    let replace_text = "Taken 1-3\n";

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  ./thief --steal [--prefix \"notes\"] [--suffix \".txt\"] [-i \"folder|file\"] [--encrypt \"password\"]");
        println!("  ./thief --restore [--decrypt \"password\"]");
        return Ok(());
    }

    let mut current_action: Option<Action> = None;
    let mut ignore_list: Vec<String> = vec!["target".to_string(), ".git".to_string()];
    let mut file_prefix: Option<String> = None;
    let mut file_suffix: Option<String> = None;
    let mut password: Option<String> = None;

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
                    for item in args[i + 1].split('|') {
                        let trimmed = item.trim().to_string();
                        if !trimmed.is_empty() && !ignore_list.contains(&trimmed) {
                            ignore_list.push(trimmed);
                        }
                    }
                    i += 1;
                }
            }
            "--prefix" => {
                if i + 1 < args.len() {
                    file_prefix = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--suffix" => {
                if i + 1 < args.len() {
                    file_suffix = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--encrypt" | "--decrypt" => {
                if i + 1 < args.len() {
                    password = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let action = current_action.ok_or("Invalid or missing action. Use '--steal' or '--restore'.")?;

    match action {
        Action::Steal => {
            let mut files = Vec::new();
            println!("Crawling target path: {:?}", target_path);
            crawl(&target_path, &mut files, &ignore_list)?;

            println!("Backing up, encrypting, and replacing files...");
            stealer(
                &files,
                &backup_folder,
                replace_text,
                file_prefix.as_deref(),
                file_suffix.as_deref(),
                password.as_deref(),
            )?;
            println!("Done!");
        }
        Action::Restoration => {
            println!("Restoring and decrypting files from backup...");
            restoration(&backup_folder, password.as_deref())?;
            println!("Restoration complete!");
        }
    }

    Ok(())
}
