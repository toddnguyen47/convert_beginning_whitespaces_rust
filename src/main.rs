use convert_beginning_whitespaces_rust::{convert, myargs};

fn main() {
    let args = myargs::get_args();
    convert::convert(&args);
}
