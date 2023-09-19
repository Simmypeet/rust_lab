pub fn count_digit(str: &str) -> usize {
    let mut digit_count = 0;

    for char in str.chars() {
        if char.is_ascii_digit() {
            digit_count += 1;
        }
    }

    digit_count
}

pub fn count_digit_r(str: &str) -> usize {
    let mut chars = str.chars();
    if let Some(char) = chars.next() {
        let str = chars.as_str();
        count_digit_r(str) + if char.is_ascii_digit() { 1 } else { 0 }
    } else {
        0
    }
}

pub fn count_digit_v2(str: &str) -> Vec<(&str, usize)> {
    let words = str.split_whitespace();

    let mut word_and_count_tuples = Vec::new();

    for word in words {
        word_and_count_tuples.push((word, count_digit(word)));
    }

    word_and_count_tuples
}

#[cfg(test)]
mod tests {
    use crate::{count_digit, count_digit_r, count_digit_v2};

    fn validate(f: impl Fn(&str) -> usize) {
        // empty string case
        assert_eq!(f(""), 0);

        // only alphabets
        assert_eq!(f("abcd"), 0);

        // alphabets with digits and spaces
        assert_eq!(f("ab12xy5   7x83y5z"), 7);

        // only digits
        assert_eq!(f("55093"), 5);

        // digits with whitespaces
        assert_eq!(f("55 0 9  3"), 5);
    }

    #[test]
    fn test_digits_count() {
        validate(count_digit);
        validate(count_digit_r);
    }

    #[test]
    fn test_digits_count2() {
        // empty string case
        assert_eq!(count_digit_v2(""), []);

        // two words case
        assert_eq!(
            count_digit_v2("ab12xy5   7x83y5z"),
            [("ab12xy5", 3), ("7x83y5z", 4)]
        );

        // one words case
        assert_eq!(count_digit_v2("ab12xy5 "), [("ab12xy5", 3)]);

        // only digits one word case
        assert_eq!(count_digit_v2("68420"), [("68420", 5)]);

        // only digits multiple words case
        assert_eq!(count_digit_v2("68420 555"), [("68420", 5), ("555", 3)]);
    }
}
