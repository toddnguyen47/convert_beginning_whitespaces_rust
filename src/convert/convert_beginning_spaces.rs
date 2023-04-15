use crate::convert::constants;
use crate::convert::convert_base;
use crate::myargs;

use super::convert_base::ConvertLine;

pub fn convert(args: &myargs::MyArgs) {
    let convert_beginning_spaces = ConvertBeginningSpaces {};
    convert_base::convert_base(args, convert_beginning_spaces);
}

struct ConvertBeginningSpaces {}

impl ConvertLine for ConvertBeginningSpaces {
    fn convert(&self, args: &myargs::MyArgs, line: &str) -> String {
        // Check the line to see if it's an empty line
        if line.trim().is_empty() {
            return String::from("");
        }

        let mut count_tab = 0;
        let mut count_other_whitespace = 0;
        let mut first_char_index = 0;
        let mut comment_char = "";
        if let Some(c1) = &args.comment_char {
            comment_char = c1;
        }
        let mut comment_line = false;
        let mut index = 0;
        for c1 in line.chars() {
            if c1 == constants::CHAR_TAB {
                count_tab += 1;
            } else if c1.is_whitespace() {
                count_other_whitespace += 1;
            } else {
                first_char_index = index;
                if !comment_char.eq_ignore_ascii_case("")
                    && c1.to_string().eq_ignore_ascii_case(comment_char)
                {
                    comment_line = true;
                    if count_other_whitespace > 0 {
                        count_other_whitespace -= 1;
                    }
                }
                break;
            }
            index += 1;
        }

        let cur_new_str_trimmed = line[first_char_index..].trim_end();
        let mut new_str = vec![constants::STR_TAB; count_tab];
        let mut num_spaces = args.num_spaces;
        if num_spaces == 0 {
            num_spaces = 1;
        }
        let quotient = (count_other_whitespace / (num_spaces)) as u32;
        for _ in 0..quotient {
            new_str.push(constants::STR_TAB);
        }
        let beginning_ws = new_str.join("");
        let results: String;
        if comment_line {
            results = beginning_ws + constants::STR_SPACE + cur_new_str_trimmed;
        } else {
            results = beginning_ws + cur_new_str_trimmed;
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        convert::{convert_base::ConvertLine, convert_beginning_spaces::ConvertBeginningSpaces},
        myargs::{self, MyArgs},
    };

    #[test]
    fn test_given_tabs_with_spaces_then_convert_properly() {
        // -- ARRANGE --
        let args: MyArgs = MyArgs {
            num_spaces: 4,
            ws_from: myargs::WhitespaceChoices::Space,
            comment_char: Option::Some("*".to_string()),
            files: vec!["".to_string()],
        };
        let str1 = "	  	hello world";
        let sut_convert = ConvertBeginningSpaces {};
        // -- ACT --
        let new_line = sut_convert.convert(&args, str1);
        // -- ARRAGE --
        let expected = "		hello world";
        assert_eq!(expected, new_line);
    }

    #[test]
    fn test_given_tabs_with_five_spaces_then_convert_properly() {
        // -- ARRANGE --
        let args: MyArgs = MyArgs {
            num_spaces: 4,
            ws_from: myargs::WhitespaceChoices::Space,
            comment_char: Option::Some("*".to_string()),
            files: vec!["".to_string()],
        };
        let str1 = "	     	hello 	world";
        let sut_convert = ConvertBeginningSpaces {};
        // -- ACT --
        let new_line = sut_convert.convert(&args, str1);
        // -- ARRAGE --
        let expected = "			hello 	world";
        assert_eq!(expected, new_line);
    }

    #[test]
    fn test_given_comment_char_then_have_an_extra_space() {
        // -- ARRANGE --
        let args: MyArgs = MyArgs {
            num_spaces: 4,
            ws_from: myargs::WhitespaceChoices::Space,
            comment_char: Option::Some("*".to_string()),
            files: vec!["".to_string()],
        };
        let str1 = "	     	 * hello 	world";
        let sut_convert = ConvertBeginningSpaces {};
        // -- ACT --
        let new_line = sut_convert.convert(&args, str1);
        // -- ARRAGE --
        let expected = "			 * hello 	world";
        assert_eq!(expected, new_line);
    }

    #[test]
    fn test_given_comment_char_with_num_spaces_2_then_have_an_extra_space() {
        // -- ARRANGE --
        let args: MyArgs = MyArgs {
            num_spaces: 2,
            ws_from: myargs::WhitespaceChoices::Space,
            comment_char: Option::Some("*".to_string()),
            files: vec!["".to_string()],
        };
        let str1 = "	     	 * hello 	world";
        let sut_convert = ConvertBeginningSpaces {};
        // -- ACT --
        let new_line = sut_convert.convert(&args, str1);
        // -- ARRAGE --
        let expected = "				 * hello 	world";
        assert_eq!(expected, new_line);
    }

    #[test]
    fn test_given_comment_char_with_just_one_tab_then_have_an_extra_space() {
        // -- ARRANGE --
        let args: MyArgs = MyArgs {
            num_spaces: 2,
            ws_from: myargs::WhitespaceChoices::Space,
            comment_char: Option::Some("*".to_string()),
            files: vec!["".to_string()],
        };
        let str1 = "	* hello 	world";
        let sut_convert = ConvertBeginningSpaces {};
        // -- ACT --
        let new_line = sut_convert.convert(&args, str1);
        // -- ARRAGE --
        let expected = "	 * hello 	world";
        assert_eq!(expected, new_line);
    }
}
