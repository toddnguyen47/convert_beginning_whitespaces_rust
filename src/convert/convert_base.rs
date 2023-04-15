use std::fs;

use crate::{convert::constants, myargs};

pub trait ConvertLine {
    /// Convert beginning space/tab accordingly
    fn convert(&self, args: &myargs::MyArgs, line: &str) -> String;
}

pub fn convert_base(args: &myargs::MyArgs, input_trait: impl ConvertLine) {
    for file in &args.files {
        let contents = fs::read_to_string(file).expect("Should have been able to read the file");
        let lines = contents.lines();
        let mut new_lines: Vec<String> = vec![];
        for line in lines {
            let new_line = input_trait.convert(args, line);
            new_lines.push(new_line)
        }
        fs::write(file, new_lines.join(constants::STR_NEWLINE))
            .expect("Should have been able to write the file");
        println!("Finished writing to: {}", file);
    }
}
