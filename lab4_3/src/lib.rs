pub fn quote(s: &str, c: char) -> String {
    format!("{}{}{}", c, s, c)
}

pub fn quote_list(v: &[&str], c: char) -> Vec<String> {
    let mut result = Vec::new();
    for str in v {
        result.push(format!("{c}{str}{c}"));
    }
    result
}

#[test]
fn test_quotes() {
    assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote_list(&[""; 0], '*'), &[""; 0]);
    assert_eq!(quote_list(&["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_recursion_start(&[""; 0], '*'), &[""; 0]);
    assert_eq!(
        quote_list_recursion_start(&["abcd", "xyz"], '*'),
        ["*abcd*", "*xyz*"]
    );
}

pub fn quote_list_recursion_start(v: &[&str], c: char) -> Vec<String> {
    quote_list_recursion(v, c, vec![], 0)
}

fn quote_list_recursion(v: &[&str], c: char, mut result: Vec<String>, index: usize) -> Vec<String> {
    if index == v.len() {
        result
    } else {
        result.push(format!("{}{}{}", c, v[index], c));
        quote_list_recursion(v, c, result, index + 1)
    }
}

pub trait Test<'a> {
    type Inner;
}

impl<'a, T> Test<'a> for T {
    type Inner = &'a i32;
}

pub struct MyStruct<'a, T>
where
    T: Test<'a, Inner = &'a i32>,
{
    pub a: &'a T,
}

fn test(a: MyStruct<i32>) {}
