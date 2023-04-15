use crate::myargs;

mod constants;
mod convert_base;
mod convert_beginning_spaces;
mod convert_beginning_tabs;

pub fn convert(args: &myargs::MyArgs) {
    match args.ws_from {
        myargs::WhitespaceChoices::Space => convert_beginning_spaces::convert(args),
        myargs::WhitespaceChoices::Tab => convert_beginning_tabs::convert(args),
    }
}
