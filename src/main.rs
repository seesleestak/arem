use indicatif::{ProgressBar, ProgressStyle};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let paths = &args[1..];

    if paths.len() > 0 {
        let length = paths.len() as u64;

        let pb = ProgressBar::new(length);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("Removing [{bar}] {pos}/{len}: {msg}")
                .progress_chars("=> "),
        );

        for path in paths {
            pb.set_message(format!("{:?}", path));
            match fs::remove_file(path) {
                Err(_) => println!("No such file or directory"),
                Ok(_) => {
                    pb.inc(1);
                }
            }
        }
        pb.finish_with_message("Complete");
    }
}
