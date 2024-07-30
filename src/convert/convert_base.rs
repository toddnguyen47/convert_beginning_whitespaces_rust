use std::fs;

use crate::{convert::constants, myargs};

pub trait ConvertLine {
    /// Convert beginning space/tab accordingly
    fn convert(&self, args: &myargs::MyArgs, line: &str) -> String;
}

pub fn convert_base(args: &myargs::MyArgs, input_trait: impl ConvertLine) {
    for file in &args.files {
        let contents = fs::read_to_string(file).expect("Should have been able to read the file");
        println!("Finished reading from: {}", file);
        let lines = contents.lines();
        let mut new_lines: Vec<String> = vec![];
        for line in lines {
            let new_line = input_trait.convert(args, line);
            new_lines.push(new_line);
        }
        // Add a newline at the end of the file if it didn't have one
        let len1 = new_lines.len();
        if len1 > 0 && !new_lines[len1 - 1].trim().is_empty() {
            new_lines.push(String::from(""));
        }
        fs::write(file, new_lines.join(constants::STR_NEWLINE))
            .expect("Should have been able to write the file");
        println!("Finished writing to: {}", file);
    }
}
