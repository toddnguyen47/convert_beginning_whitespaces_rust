use clap::{Parser, ValueEnum};

/// Convert beginning whitespaces from space to tab, or vice versa.
///
/// Example
/// ```
/// convert_beginning_whitespaces_rust --ws-from space --comment-char "*" --num-spaces 4
/// ```
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// Number of spaces to convert into / from
    #[arg(short, long, default_value_t = 4)]
    pub num_spaces: i32,

    /// Convert from spaces or from tabs
    #[arg(short, long, value_enum)]
    pub ws_from: WhitespaceChoices,

    /// Optional: character that starts a multi-line comment
    #[arg(short, long)]
    pub comment_char: Option<String>,

    /// Files to operate on
    pub files: Vec<String>,
}

impl MyArgs {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.num_spaces <= 0 {
            return Err("num spaces has to be at least 1");
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WhitespaceChoices {
    Space,
    Tab,
}

pub fn get_args() -> MyArgs {
    let args = MyArgs::parse();
    args
}

#[cfg(test)]
mod tests {
    use crate::myargs::WhitespaceChoices;

    use super::MyArgs;

    #[test]
    fn test_given_num_spaces_0_then_return_err() {
        // -- ARRANGE --
        let args = MyArgs {
            num_spaces: 0,
            ws_from: WhitespaceChoices::Space,
            comment_char: Some("".to_string()),
            files: vec!["".to_string()],
        };
        // -- ACT --
        let res = args.validate();
        // -- RESULT --
        assert!(res.is_err())
    }

    #[test]
    fn test_given_num_spaces_neg_1_then_return_err() {
        // -- ARRANGE --
        let args = MyArgs {
            num_spaces: -1,
            ws_from: WhitespaceChoices::Space,
            comment_char: Some("".to_string()),
            files: vec!["".to_string()],
        };
        // -- ACT --
        let res = args.validate();
        // -- RESULT --
        assert!(res.is_err())
    }

    #[test]
    fn test_given_num_spaces_1_then_return_ok() {
        // -- ARRANGE --
        let args = MyArgs {
            num_spaces: 1,
            ws_from: WhitespaceChoices::Space,
            comment_char: Some("".to_string()),
            files: vec!["".to_string()],
        };
        // -- ACT --
        let res = args.validate();
        // -- RESULT --
        assert!(res.is_ok())
    }
}
