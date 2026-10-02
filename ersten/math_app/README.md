# Thief 🦀📂

A cross-platform file backup, encryption, and replacement utility written in Rust. It recursively crawls a target directory, filters files by prefix/suffix and custom ignore rules, backs them up (with optional AES-256 encryption), overwrites the originals, and supports full automated restoration.

---

## Features
- **Cross-Platform:** Automatically detects the user's home directory on Linux, macOS, and Windows.
- **Selective Filtering:** Target specific files using `--prefix` and/or `--suffix` (e.g., extensions like `.txt` or `.py`).
- **Dynamic Ignores:** Exclude files or folders on the fly using `-i` or `--ignore`.
- **AES-256-GCM Encryption:** Secure your backups using a password-derived key.
- **Self-Contained Backups:** Every backup file safely retains its absolute path of origin.

---

## Prerequisites & Setup

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
aes-gcm = "0.10"
sha2 = "0.10"
rand = "0.8"
