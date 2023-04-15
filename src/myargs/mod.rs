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
    pub num_spaces: u32,

    /// Convert from spaces or from tabs
    #[arg(short, long, value_enum)]
    pub ws_from: WhitespaceChoices,

    /// Optional: character that starts a multi-line comment
    #[arg(short, long)]
    pub comment_char: Option<String>,

    /// Files to operate on
    pub files: Vec<String>,
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
