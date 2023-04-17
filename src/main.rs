use convert_beginning_whitespaces_rust::{convert, myargs};

fn main() {
    let args = myargs::get_args();
    if let Err(err) = args.validate() {
        panic!("ERROR: {}", err);
    }
    convert::convert(&args);
}
