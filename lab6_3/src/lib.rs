use std::borrow::Borrow;

fn join_strings_internal<T: Borrow<str>>(str: &[T], pattern: &str) -> String {
    let mut result = String::new();

    for (i, s) in str.iter().enumerate() {
        result.push_str(s.borrow());

        if i != str.len() - 1 {
            result.push_str(pattern);
        }
    }

    result
}

pub fn join_strings(str: &[&str], pattern: &str) -> String {
    join_strings_internal(str, pattern)
}

pub fn join_numbers(nums: &[i32], pattern: &str) -> String {
    join_strings_internal(
        &nums.iter().map(i32::to_string).collect::<Vec<String>>(),
        pattern,
    )
}

#[cfg(test)]
mod tests {

    use crate::{join_numbers, join_strings};

    #[test]
    fn join_strings_test() {
        // empty string array case
        assert_eq!(join_strings(&[], ""), "");

        // empty pattern join
        assert_eq!(
            join_strings(&["yo hello", "what", "brah"], ""),
            "yo hellowhatbrah"
        );

        // one string join
        assert_eq!(join_strings(&["yo hello"], " "), "yo hello");

        // normal case
        assert_eq!(
            join_strings(&["yo hello", "what", "brah"], "-:"),
            "yo hello-:what-:brah"
        );

        assert_eq!(join_strings(&[], ","), "");
        assert_eq!(join_strings(&["C"], ","), "C");
        let patterns = ["C", "Rust", "C++", "Python"];
        assert_eq!(join_strings(&patterns, ", "), "C, Rust, C++, Python");
        assert_eq!(join_strings(&patterns, ";;"), "C;;Rust;;C++;;Python");
    }

    #[test]
    fn join_numbers_test() {
        // empty string array case
        assert_eq!(join_numbers(&[], ""), "");

        // empty pattern join
        assert_eq!(join_numbers(&[3, 2, -1], ""), "32-1");

        // one string join
        assert_eq!(join_numbers(&[3], " "), "3");

        // normal case
        assert_eq!(join_numbers(&[32, 2, -60], "-:"), "32-:2-:-60");

        assert_eq!(join_numbers(&[], ","), "");
        assert_eq!(join_numbers(&[25], ","), "25");
        let patterns = [5, 10, -1, 2];
        assert_eq!(join_numbers(&patterns, ", "), "5, 10, -1, 2");
        assert_eq!(join_numbers(&patterns, ";;"), "5;;10;;-1;;2");
    }
}
