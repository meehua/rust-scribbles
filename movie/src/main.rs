use std::{error::Error, process};

use movie::read_text_to_json;
use rfd::FileDialog;

fn main() -> Result<(), Box<dyn Error>> {
    match FileDialog::new()
        .add_filter("Text File", &["txt"])
        .set_title("Select a text file")
        .set_directory("./")
        .pick_file()
    {
        Some(filepath) => {
            let saved_path = read_text_to_json(&filepath);
            println!("Selected file and saved it: {saved_path:?}");
            Ok(())
        }
        None => {
            eprintln!("No file selected");
            process::exit(-1)
        }
    }
}
