use walkdir::WalkDir;

fn main() {
    println!("Scanning current directory for files...");

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            println!("Found file {}", entry.path().display());
        }
    }
}
