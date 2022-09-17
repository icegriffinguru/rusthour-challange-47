use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn visit_all_files_and_count_lines() {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        
        let file_path = entry.path();
        // println!("{:?}", file_path);

        // if file name ends with ".txt"
        if file_path.to_str().unwrap().ends_with(".txt") {
            let file = BufReader::new(File::open(file_path).expect("Unable to open file"));
            let mut cnt  = 0;
            
            for _ in file.lines() {
                cnt = cnt + 1;
            }

            println!("File Path: {:?};\t\t\tNumber of Lines: {:?}", file_path.display(), cnt);
        }
    }
}

fn main() {
    println!("--- Number of lines in all txt files ---");
    visit_all_files_and_count_lines();
}
