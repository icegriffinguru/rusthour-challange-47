use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, BufRead, self, Write};

fn visit_all_files_and_count_lines() -> io::Result<()> {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {        
        let file_path = entry.path();

        // if file name ends with ".txt"
        if file_path.to_str().unwrap().ends_with(".txt") {
            let file = BufReader::new(File::open(file_path).expect("Unable to open file"));
            let mut cnt  = 0;
            
            for _ in file.lines() {
                cnt = cnt + 1;
            }

            io::stdout().write_fmt(format_args!("File Path: {:?};\t\t\tNumber of Lines: {:?}\n", file_path.display(), cnt))?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()>{
    io::stdout().write_fmt(format_args!("--- Number of lines in all txt files ---\n"))?;
    visit_all_files_and_count_lines()?;

    Ok(())
}
